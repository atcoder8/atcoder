use itertools::enumerate;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();

    let mut cnt = 0_usize;
    for (j, &c) in enumerate(&s) {
        if c != 'B' {
            continue;
        }

        for interval in 1..=j.min(n - 1 - j) {
            if s[j - interval] == 'A' && s[j + interval] == 'C' {
                cnt += 1;
            }
        }
    }

    println!("{}", cnt);
}
