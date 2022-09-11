use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    println!("{}", solve());
}

fn solve() -> String {
    input! {
        (n, s): (usize, Chars),
    }

    if s.iter().all(|&c| c == 'd') {
        return s.iter().collect();
    }

    let l = s.iter().find_position(|&&c| c == 'p').unwrap().0;

    let mut min_chars = s.clone();

    for r in l..(n + 1) {
        let mut replaced = vec![];
        for &c in s[..l].iter() {
            replaced.push(c);
        }
        for &c in s[l..r].iter().rev() {
            if c == 'p' {
                replaced.push('d');
            } else {
                replaced.push('p');
            }
        }
        for &c in s[r..].iter() {
            replaced.push(c);
        }

        if replaced < min_chars {
            min_chars = replaced;
        }
    }

    min_chars.iter().collect()
}
