use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let ans = (0..t)
        .map(|_| {
            input! {
                (n, m): (usize, usize),
            }

            if solve(n, m) {
                "Alice"
            } else {
                "Bob"
            }
        })
        .join("\n");
    println!("{}", ans);
}

fn solve(n: usize, m: usize) -> bool {
    if n * (n + 1) <= m {
        return true;
    }

    let t = n * (n + 1) % m;
    !(1 <= t && t <= n)
}
