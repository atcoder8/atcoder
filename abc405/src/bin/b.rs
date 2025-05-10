use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, m): (usize, usize),
        aa: [Usize1; n],
    }

    let mut counts = vec![0_usize; m];
    for &a in &aa {
        counts[a] += 1;
    }

    if counts.iter().any(|&cnt| cnt == 0) {
        return 0;
    }

    for (i, &a) in aa.iter().rev().enumerate() {
        counts[a] -= 1;
        if counts[a] == 0 {
            return i + 1;
        }
    }

    panic!();
}
