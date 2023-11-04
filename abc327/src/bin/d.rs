use std::ops::Not;

use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        aa: [Usize1; m],
        bb: [Usize1; m],
    }

    let mut a_to_idx = vec![vec![]; n];
    for (i, &a) in enumerate(&aa) {
        a_to_idx[a].push(i);
    }

    let mut b_to_idx = vec![vec![]; n];
    for (i, &b) in enumerate(&bb) {
        b_to_idx[b].push(i);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Parity {
        Zero,
        One,
    }

    impl Not for Parity {
        type Output = Self;

        fn not(self) -> Self::Output {
            match self {
                Parity::Zero => Parity::One,
                Parity::One => Parity::Zero,
            }
        }
    }

    let mut parities: Vec<Option<Parity>> = vec![None; n];
    for start in 0..n {
        if parities[start].is_some() {
            continue;
        }

        let mut stack = vec![(start, Parity::Zero)];
        while let Some((cur, parity)) = stack.pop() {
            if let Some(p) = parities[cur] {
                if p == parity {
                    continue;
                }

                return false;
            }

            parities[cur] = Some(parity);

            for &idx in &a_to_idx[cur] {
                stack.push((bb[idx], !parity));
            }

            for &idx in &b_to_idx[cur] {
                stack.push((aa[idx], !parity));
            }
        }
    }

    true
}
