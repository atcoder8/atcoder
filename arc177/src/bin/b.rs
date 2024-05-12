use itertools::enumerate;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        states: Chars,
    }

    let mut ans = String::new();
    for (pos, state) in enumerate(states).rev() {
        if state == '0' {
            continue;
        }

        ans.extend("A".repeat(pos + 1).chars());
        ans.extend("B".repeat(pos).chars());
    }

    println!("{}\n{}", ans.len(), ans);
}
