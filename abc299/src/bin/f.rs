// unfinished

use im_rc::HashSet;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut used = HashSet::new();

    for i in 0..n {
        for j in (i + 1)..=n {
            let sub = s[i..j].to_owned();
            if 2 * j - i <= n && s[j..(2 * j - i)] == sub {
                used.insert(sub);
            }
        }
    }

    println!("{}", used.len());
}
