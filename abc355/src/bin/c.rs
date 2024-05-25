use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (n, t): (usize, usize),
        aa: [Usize1; t],
    }

    let mut hor_counts = vec![0; n];
    let mut ver_counts = vec![0; n];
    let mut diagonal1 = 0;
    let mut diagonal2 = 0;

    for (i, &a) in enumerate(&aa) {
        let (row, col) = (a / n, a % n);

        hor_counts[row] += 1;
        ver_counts[col] += 1;

        if row == col {
            diagonal1 += 1;
        }

        if row == n - 1 - col {
            diagonal2 += 1;
        }

        if hor_counts[row] == n || ver_counts[col] == n || diagonal1 == n || diagonal2 == n {
            return Some(i + 1);
        }
    }

    None
}
