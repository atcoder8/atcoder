use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
        pp: [Usize1; n - 1],
    }

    let mut graph = vec![vec![]; n];
    for (i, &p) in enumerate(&pp) {
        graph[p].push(i + 1);
    }

    let mut coefficients = vec![0; n];
    let mut stack = vec![(0, 0)];
    while let Some((cur, deg)) = stack.pop() {
        coefficients[deg] += aa[cur];

        for &next in &graph[cur] {
            stack.push((next, deg + 1));
        }
    }

    let last = coefficients.iter().rev().find(|&&coef| coef != 0);
    let ans = match last {
        Some(&last) => {
            if last < 0 {
                '-'
            } else {
                '+'
            }
        }
        None => '0',
    };
    println!("{}", ans);
}
