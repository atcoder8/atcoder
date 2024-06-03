use itertools::{chain, enumerate, Itertools};
use proconio::input;

fn main() {
    let ans = match solve() {
        Some(ans) => format!("Yes\n{}", ans.iter().join(" ")),
        None => "No".to_string(),
    };
    println!("{}", ans);
}

fn solve() -> Option<Vec<i64>> {
    input! {
        (n, k): (usize, i64),
        aa: [i64; n],
    }

    let mut pos = vec![];
    let mut neg = vec![];
    for &a in &aa {
        if a >= 0 {
            pos.push(a);
        } else {
            neg.push(a);
        }
    }

    let is_good = |seq: &[i64]| {
        let mut acc = vec![0; n + 1];
        for (i, &elem) in enumerate(seq) {
            acc[i + 1] = acc[i] + elem;
        }

        let Some(right_lt) = (0..=n).rev().position(|i| acc[i] < k) else { return true; };
        let Some(left_ge) = (0..=n).position(|i| acc[i] >= k) else { return true; };

        right_lt < left_ge
    };

    {
        let bb = chain(&neg, &pos).cloned().collect_vec();
        if is_good(&bb) {
            return Some(bb);
        }
    }

    {
        let bb = chain(&pos, &neg).cloned().collect_vec();
        if is_good(&bb) {
            return Some(bb);
        }
    }

    None
}
