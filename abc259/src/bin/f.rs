use std::cmp::Reverse;

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let dd: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let mut uvw: Vec<(usize, usize, i64)> = Vec::new();
    for _ in 0..(n - 1) {
        uvw.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse().unwrap(),
            )
        });
    }

    let mut graph = vec![vec![]; n as usize];
    for (u, v, w) in uvw {
        graph[u as usize].push((v, w));
        graph[v as usize].push((u, w));
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let max_score = rec(&graph, &mut vec![None; n], &dd, None, 0);

    println!("{}", max_score.0.max(max_score.1));
}

fn rec(
    graph: &Vec<Vec<(usize, i64)>>,
    dp: &mut Vec<Option<(i64, i64)>>,
    dd: &Vec<usize>,
    parent_node: Option<usize>,
    curr_node: usize,
) -> (i64, i64) {
    let d = dd[curr_node];

    let mut scores = vec![];

    for (child_node, w) in graph[curr_node].iter() {
        if Some(*child_node) == parent_node {
            continue;
        }

        if dp[*child_node].is_none() {
            dp[*child_node] = Some(rec(graph, dp, dd, Some(curr_node), *child_node));
        }

        let (child_le, child_lt) = dp[*child_node].unwrap();

        scores.push((child_lt + *w, child_le));
    }

    scores.sort_unstable_by_key(|(connect, not_connect)| Reverse(connect - not_connect));

    let mut curr_le = 0;
    for (i, (connect, not_connect)) in scores.iter().enumerate() {
        curr_le += if i < d { connect.max(not_connect) } else { not_connect };
    }

    if d == 0 {
        return (scores.iter().map(|(_, not_connect)| not_connect).sum(), -2e18 as i64);
    }

    let mut max_score = (0, 0);

    for (i, (connect, not_connect)) in scores.into_iter().enumerate() {
        max_score.0 += if i < d {
            connect.max(not_connect)
        } else {
            not_connect
        };
        max_score.1 += if i < d - 1 {
            connect.max(not_connect)
        } else {
            not_connect
        };
    }

    max_score
}
