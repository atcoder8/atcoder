use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        n: usize,
    }

    let mut rev_s = s.iter().cloned().rev().collect_vec();
    rev_s.resize(60, '0');

    let mut less: Option<usize> = None;
    let mut equal: Option<usize> = Some(0);

    for i in (0..60).rev() {
        let c = rev_s[i];
        let bit = 1_usize << i;

        if c != '0' {
            if let Some(less) = &mut less {
                *less += bit;
            }
        }

        if n & bit != 0 && c != '1' {
            less = less.max(equal);
        }

        if c != '?' && (n >> i) & 1 != (c as u8 - b'0') as usize {
            equal = None;
        }

        if let Some(equal) = &mut equal {
            *equal += n & bit;
        }
    }

    let ans = less.max(equal);
    if let Some(ans) = ans {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
