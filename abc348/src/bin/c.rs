use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(usize, usize); n],
    }

    let mut ans = 0;
    for (_, group) in &ac.iter().sorted_unstable_by_key(|v| v.1).group_by(|v| v.1) {
        ans = ans.max(group.map(|v| v.0).min().unwrap());
    }

    println!("{}", ans);
}
