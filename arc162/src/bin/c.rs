use fixedbitset::FixedBitSet;
use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    println!(
        "{}",
        (0..t)
            .map(|_| if solve() { "Alice" } else { "Bob" })
            .join("\n")
    );
}

fn solve() -> bool {
    input! {
        (n, k): (usize, usize),
        pp: [Usize1; n - 1],
        aa: [i64; n],
    }

    let mut graph = vec![vec![]; n];
    for (i, &p) in enumerate(&pp) {
        graph[p].push(i + 1);
    }

    #[derive(Debug, Clone)]
    struct Status {
        exists: FixedBitSet,
        empty_num: usize,
    }

    let mut statuses = vec![
        Status {
            exists: FixedBitSet::with_capacity(k + 1),
            empty_num: 0
        };
        n
    ];

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum DFSNode {
        Forward(usize),
        Back(usize),
    }

    let mut stack = vec![DFSNode::Forward(0)];
    while let Some(dfs_node) = stack.pop() {
        match dfs_node {
            DFSNode::Forward(cur) => {
                stack.push(DFSNode::Back(cur));
                stack.extend(graph[cur].iter().map(|&next| DFSNode::Forward(next)));
            }
            DFSNode::Back(cur) => {
                if aa[cur] == -1 {
                    statuses[cur].empty_num += 1;
                } else {
                    let a = aa[cur] as usize;
                    if a <= k {
                        statuses[cur].exists.insert(a);
                    }
                }

                for &next in &graph[cur] {
                    let child_exists = statuses[next].exists.clone();
                    statuses[cur].exists.union_with(&child_exists);
                    statuses[cur].empty_num += statuses[next].empty_num;
                }
            }
        };
    }

    let is_ok = |status: &Status| {
        if status.exists[k] || status.empty_num >= 2 {
            return false;
        }

        let shortage_num = k - status.exists.count_ones(0..k);

        shortage_num == 0 || (shortage_num == 1 && status.empty_num == 1)
    };

    statuses.iter().any(|status| is_ok(&status))
}
