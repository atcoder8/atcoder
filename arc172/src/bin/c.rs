// unfinished

use std::cmp::Ordering;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        cc: Chars,
    }

    let mut ans = 0;
    let mut a = 0;
    let mut b = 0;
    let mut prev = (
        Ordering::Equal,
        if cc[0] == 'A' {
            Ordering::Greater
        } else {
            Ordering::Less
        },
    );
    for &c in &cc[1..] {
        if c == 'A' {
            a += 1;
        } else {
            b += 1;
        }

        let cur = (
            a.cmp(&b),
            if cc[0] == 'A' {
                (a + 1).cmp(&b)
            } else {
                a.cmp(&(b + 1))
            },
        );

        ans += (prev.0 != cur.1 || prev.1 != cur.0) as usize;

        prev = cur;
    }

    println!("{}", ans);
}
