use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{derive_readable, input, marker::Usize1};

#[derive_readable()]
#[derive(Debug, Clone, Copy)]
struct Node {
    parent: Usize1,
    node_type: usize,
    strength: usize,
    growth: usize,
}

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        mut nodes: [Node; n - 1],
    }

    let mut graph = vec![vec![]; n];
    for (i, node) in nodes.iter().enumerate() {
        graph[i + 1].push(node.parent);
        graph[node.parent].push(i + 1);
    }

    let portion_node_indexes = (1..n)
        .filter(|&i| nodes[i - 1].node_type == 2)
        .collect_vec();
    let portion_num = portion_node_indexes.len();

    if portion_num == n - 1 {
        return true;
    }

    let max_strength = nodes
        .iter()
        .filter(|node| node.node_type == 1)
        .map(|node| node.strength)
        .max()
        .unwrap();

    nodes.insert(
        0,
        Node {
            parent: 0,
            node_type: 1,
            strength: 0,
            growth: 0,
        },
    );

    let search =
        |mut power: usize, used_portions: usize, earned: &[bool]| -> (usize, usize, Vec<bool>) {
            let mut visited = vec![false; n];
            let mut heap = BinaryHeap::from(vec![(Reverse(0), 0)]);
            let mut accessible_portions = 0;

            while let Some((_, cur)) = heap.pop() {
                if visited[cur] {
                    continue;
                }

                let node = nodes[cur];

                if node.node_type == 1 {
                    if power < node.strength {
                        break;
                    }

                    if !earned[cur] {
                        power = (power + node.growth).min(max_strength);
                    }
                } else {
                    let pos = portion_node_indexes
                        .iter()
                        .position(|&node_idx| node_idx == cur)
                        .unwrap();
                    accessible_portions |= 1 << pos;

                    if (used_portions >> pos) & 1 == 0 {
                        continue;
                    }
                }

                visited[cur] = true;

                for &next in &graph[cur] {
                    heap.push((Reverse(nodes[next].strength), next));
                }
            }

            (power, accessible_portions, visited)
        };

    let mut dp = vec![None; 1 << portion_num];
    dp[0] = Some(search(1, 0, &vec![false; n]));
    for bit in 0..(1 << portion_num) {
        let (power, accessible_portions, earned) = match dp[bit].clone() {
            Some((power, accessible_portions, earned)) => (power, accessible_portions, earned),
            None => continue,
        };

        for portion_pos in 0..portion_num {
            if (bit >> portion_pos) & 1 == 1 || (accessible_portions >> portion_pos) & 1 == 0 {
                continue;
            }

            let portion = portion_node_indexes[portion_pos];
            let enhanced_power = (power * nodes[portion].growth).min(max_strength);

            let next_bit = bit | 1 << portion_pos;
            let candidate = search(enhanced_power, next_bit, &earned);
            if dp[next_bit].is_none() || candidate.0 > dp[next_bit].as_ref().unwrap().0 {
                dp[next_bit] = Some(candidate);
            }
        }
    }

    dp.iter()
        .filter_map(|x| x.as_ref())
        .map(|x| x.0)
        .max()
        .unwrap()
        == max_strength
}
