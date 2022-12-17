use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &uv {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph.iter_mut().for_each(|x| x.sort_unstable());

    let mut parities: Vec<Option<bool>> = vec![None; n];
    let mut ret = 0;
    let mut group_comb = 0;

    for root in 0..n {
        if parities[root].is_some() {
            continue;
        }

        let mut queue: VecDeque<(Option<usize>, usize)> = VecDeque::new();
        queue.push_back((None, root));
        parities[root] = Some(false);
        let mut group_size = 1;
        let mut even_cnt = 1;
        let mut odd_cnt = 0;

        while let Some((par_node, cur_node)) = queue.pop_front() {
            let cur_parity = parities[cur_node].unwrap();

            for &next_node in &graph[cur_node] {
                if Some(next_node) == par_node {
                    continue;
                }

                if let Some(next_parity) = parities[next_node] {
                    if next_parity == cur_parity {
                        return 0;
                    }
                } else {
                    let next_parity = !cur_parity;
                    parities[next_node] = Some(next_parity);
                    group_size += 1;
                    queue.push_back((Some(cur_node), next_node));

                    if next_parity {
                        odd_cnt += 1;
                    } else {
                        even_cnt += 1;
                    }
                }
            }
        }

        ret += even_cnt * odd_cnt;
        group_comb += group_size * (n - group_size);
    }

    ret += group_comb / 2;
    ret -= uv.len();

    ret
}
