use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let ans = (0..n)
        .permutations(2)
        .map(|perm| ss[perm[0]].to_string() + &ss[perm[1]])
        .sorted_unstable()
        .dedup()
        .count();
    println!("{}", ans);
}
