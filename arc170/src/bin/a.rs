use std::iter::zip;

use proconio::{input, marker::Chars};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    }

    if zip(&s, &t)
        .take_while(|(_, &c2)| c2 == 'B')
        .any(|(&c1, &c2)| c1 == 'A' && c2 == 'B')
        || zip(&s, &t)
            .rev()
            .take_while(|(_, &c2)| c2 == 'A')
            .any(|(&c1, &c2)| c1 == 'B' && c2 == 'A')
    {
        return None;
    }

    let mut ans = 0;
    let mut cnt_b_to_a = 0;
    for (&c1, &c2) in zip(&s, &t) {
        if c1 == c2 {
            continue;
        }

        if c1 == 'A' {
            // A -> B
            if cnt_b_to_a != 0 {
                cnt_b_to_a -= 1;
            } else {
                ans += 1;
            }
        } else {
            // B -> A
            cnt_b_to_a += 1;
            ans += 1;
        }
    }

    Some(ans)
}
