use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, m): (usize, usize),
        s: Chars,
        t: Chars,
    }

    let mut deleted = vec![false; n];
    let mut used = vec![false; n];
    let mut stack = (0..=n - m).filter(|&i| s[i..i + m] == t).collect_vec();
    while let Some(delete_pos) = stack.pop() {
        if used[delete_pos] {
            continue;
        }

        used[delete_pos] = true;

        deleted[delete_pos..delete_pos + m]
            .iter_mut()
            .for_each(|x| {
                *x = true;
            });

        for i in delete_pos.saturating_sub(m - 1)..=(delete_pos + m - 1).min(n - m) {
            if !used[i] && (0..m).all(|j| deleted[i + j] || s[i + j] == t[j]) {
                stack.push(i);
            }
        }
    }

    deleted.iter().all(|&x| x)
}
