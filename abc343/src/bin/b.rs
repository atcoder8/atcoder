use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mat: [[usize; n]; n],
    }

    for i in 0..n {
        let ans = (0..n).filter(|&j| mat[i][j] == 1).map(|i| i + 1).join(" ");
        println!("{}", ans);
    }
}
