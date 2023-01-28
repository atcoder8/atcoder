use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
        tt: [Chars; m],
    }

    let tt: HashSet<Vec<char>> = tt.iter().cloned().collect();

    let ans = ss.iter().filter(|&s| tt.contains(&s[3..])).count();
    println!("{}", ans);
}
