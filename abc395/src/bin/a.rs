use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let ans = aa.iter().tuple_windows().all(|(&a1, &a2)| a1 < a2);
    println!("{}", if ans { "Yes" } else { "No" });
}
