use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(usize, usize); n],
    }

    let iac = enumerate(ac)
        .sorted_unstable_by_key(|&(_, (_, c))| c)
        .collect_vec();
    let mut ans = vec![];
    let mut max_a = 0;
    for &(i, (a, _)) in &iac {
        if a > max_a {
            ans.push(i + 1);
            max_a = a;
        }
    }
    ans.sort_unstable();

    println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
