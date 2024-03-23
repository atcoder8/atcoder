use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let ans = aa
        .into_iter()
        .tuple_windows()
        .map(|(a1, a2)| a1 * a2)
        .join(" ");
    println!("{}", ans);
}
