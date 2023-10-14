use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, t): (usize, Chars),
        ss: [Chars; n],
    }

    let is_match = |s: &Vec<char>| {
        if s.len().abs_diff(t.len()) >= 2 {
            return false;
        }

        if s == &t {
            return true;
        }

        if s.len() == t.len() {
            return s.iter().zip(&t).filter(|(&c1, &c2)| c1 != c2).count() == 1;
        }

        let (s, t) = if s.len() < t.len() { (s, &t) } else { (&t, s) };

        let mut inserted = false;
        let mut pos = 0;
        for &c in t {
            if pos == s.len() {
                break;
            }

            if s[pos] != c {
                if inserted {
                    return false;
                }

                inserted = true;
            } else {
                pos += 1;
            }
        }

        true
    };

    let mut ans = vec![];
    for (i, s) in ss.iter().enumerate() {
        if is_match(s) {
            ans.push(i + 1);
        }
    }
    println!("{}\n{}", ans.len(), ans.iter().join(" "));
}
