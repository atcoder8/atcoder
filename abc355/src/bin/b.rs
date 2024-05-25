use itertools::{chain, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        bb: [usize; m],
    }

    let ans = chain(&aa, &bb)
        .cloned()
        .sorted_unstable()
        .tuple_windows()
        .any(|(v1, v2)| aa.contains(&v1) && aa.contains(&v2));
    println!("{}", if ans { "Yes" } else { "No" });
}
