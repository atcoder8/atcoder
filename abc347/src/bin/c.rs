use itertools::Itertools;
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (n, a, b): (usize, usize, usize),
        dd: [usize; n],
    }

    if n == 1 {
        return true;
    }

    let rems = dd
        .iter()
        .map(|d| d % (a + b))
        .sorted_unstable()
        .collect_vec();
    a + b + rems[0] - rems[n - 1] > b || rems.iter().tuple_windows().any(|(&r1, &r2)| r2 - r1 > b)
}
