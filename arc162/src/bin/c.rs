// unfinished

use im_rc::HashSet;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        println!("{}", if solve() { "Alice" } else { "Bob" });
    }
}

fn solve() -> bool {
    input! {
        (n, k): (usize, usize),
        pp: [Usize1; n - 1],
        aa: [i64; n],
    }

    let aa: Vec<Option<usize>> = aa
        .iter()
        .map(|&a| if a != -1 { Some(a as usize) } else { None })
        .collect();

    let mut graph = vec![vec![]; n];
    for (i, &p) in pp.iter().enumerate() {
        graph[i + 1].push(p);
        graph[p].push(i + 1);
    }

    let mut alice_win = false;

    rec(&graph, k, &aa, None, 0, &mut alice_win);

    alice_win
}

fn rec(
    graph: &Vec<Vec<usize>>,
    k: usize,
    aa: &Vec<Option<usize>>,
    parent: Option<usize>,
    cur: usize,
    alice_win: &mut bool,
) -> Option<HashSet<usize>> {
    let mut union: HashSet<usize> = HashSet::new();
    for &next in &graph[cur] {
        if Some(next) == parent {
            continue;
        }

        if let Some(set) = rec(graph, k, aa, Some(cur), next, alice_win) {
            union.extend(set.into_iter());
        } else {
            return None;
        }
    }

    if let Some(a) = aa[cur] {
        if a == k {
            return None;
        }

        if a < k {
            union.insert(a);
        }

        if union.len() == k {
            *alice_win = true;

            return None;
        }

        return Some(union);
    }

    if union.len() == k - 1 {
        *alice_win = true;

        return None;
    }

    return None;
}
