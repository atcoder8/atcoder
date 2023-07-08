use itertools::{join, Itertools};
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        if let Some(ans) = solve() {
            println!("Yes\n{}", join(ans, " "));
        } else {
            println!("No");
        }
    }
}

fn solve() -> Option<Vec<usize>> {
    input! {
        n: usize,
    }

    if n == 2 {
        return None;
    }

    let mut aa = (1..n).map(|i| i * (i + 1)).collect_vec();
    if aa.contains(&n) {
        aa.retain(|&a| a != 6 && a != 12);
        aa.append(&mut vec![4, n * (n + 1), n + 1]);
    } else {
        aa.push(n);
    }

    Some(aa)
}
