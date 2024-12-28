use itertools::izip;
use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        _k: usize,
        mut s: Chars,
        mut t: Chars,
    }

    if s.len() < t.len() {
        std::mem::swap(&mut s, &mut t);
    }

    if s.len() - t.len() >= 2 {
        return false;
    }

    if s.len() == t.len() {
        return izip!(&s, &t).filter(|(&c1, &c2)| c1 != c2).count() <= 1;
    }

    let mut operated = false;
    let mut idx = 0_usize;
    for &c in &t {
        if s[idx] != c {
            if operated {
                return false;
            }

            operated = true;
            idx += 1;
        }

        idx += 1;
    }

    true
}
