use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        ss: [String; 12],
    }

    let ans = enumerate(&ss).filter(|&(i, s)| s.len() == i + 1).count();
    println!("{}", ans);
}
