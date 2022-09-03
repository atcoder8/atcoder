use std::{collections::VecDeque, process};

fn main() {
    let (n, m) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let aa = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };
    let mut uv = Vec::new();
    for _ in 0..m {
        uv.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        });
    }

    if m == 0 {
        println!("0");
        process::exit(0);
    }

    let mut graph = vec![vec![]; n as usize];
    for &(a, b) in uv.iter() {
        graph[a as usize].push(b);
        graph[b as usize].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let init_costs: Vec<usize> = (0..n)
        .map(|i| graph[i].iter().map(|&next| aa[next]).sum())
        .collect();

    let mut ok: usize = 2e18 as usize;
    let mut ng: usize = 0;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;

        let removable_nodes: Vec<usize> = (0..n)
            .filter(|&i| init_costs[i] <= mid)
            .collect();

        let mut used = vec![false; n];
        for &node in removable_nodes.iter() {
            used[node] = true;
        }
        let mut que = VecDeque::from(removable_nodes);

        let mut costs = init_costs.clone();

        while let Some(curr) = que.pop_front() {
            for &next in graph[curr].iter() {
                if used[next] {
                    continue;
                }

                costs[next] -= aa[curr];

                if costs[next] <= mid {
                    used[next] = true;
                    que.push_back(next);
                }
            }
        }

        if used.iter().all(|&x| x) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
