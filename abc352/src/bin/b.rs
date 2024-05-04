use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans = vec![];
    let mut idx = 0;

    for &c in &s {
        while t[idx] != c {
            idx += 1;
        }

        ans.push(idx + 1);

        idx += 1;
    }

    println!("{}", ans.iter().join(" "));
}
