use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        s: Chars,
        q: usize,
        kk: [Usize1; q],
    }

    let solve = |mut k: usize| {
        let mut max_exp = 0_usize;
        while s.len() << max_exp < k {
            max_exp += 1;
        }

        let mut parity = false;
        for exp in (0..=max_exp).rev() {
            if k < s.len() {
                break;
            }

            let shift_len = s.len() << exp;
            if k >= shift_len {
                k -= shift_len;
                parity = !parity;
            }
        }

        let mut c = s[k];
        if parity {
            if c.is_ascii_uppercase() {
                c = c.to_ascii_lowercase();
            } else {
                c = c.to_ascii_uppercase();
            }
        }

        c
    };

    println!("{}", kk.iter().map(|&k| solve(k)).join(" "));
}
