// unfinished

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        println!("{}", if solve() { "Yes" } else { "No" });
    }
}

fn solve() -> bool {
    input! {
        n: usize,
        aa: [Usize1; n],
        bb: [Usize1; n],
    }

    if aa == bb {
        return true;
    }

    let is_ok = |shift_num: usize| -> bool {
        let shifted_aa = aa[shift_num..]
            .iter()
            .chain(aa[..shift_num].iter())
            .cloned()
            .collect_vec();

        let mut replaced_aa = shifted_aa.clone();

        let start_idx = (0..n).find(|&i| {
            if replaced_aa[i] == bb[i] && replaced_aa[(i + 1) % n] == bb[i] {
                return true;
            }

            false
        });

        let start_idx = if let Some(start_idx) = start_idx {
            start_idx
        } else {
            let start_idx = (0..n).find(|&i| {
                if replaced_aa[i] == bb[i] || replaced_aa[(i % 1) % n] == bb[i] {
                    return true;
                }

                false
            });

            if let Some(start_idx) = start_idx {
                start_idx
            } else {
                return false;
            }
        };

        let mut replaced_flag = false;

        for i in 0..n {
            let cur_idx = (start_idx + n - i) % n;
            let b = bb[cur_idx];

            let behind_idx = (cur_idx + 1) % n;

            if replaced_aa[behind_idx] == b {
                replaced_aa[cur_idx] = replaced_aa[behind_idx];
                replaced_flag = true;
                continue;
            }

            if shifted_aa[behind_idx] == bb[cur_idx] {
                replaced_aa[cur_idx] = shifted_aa[behind_idx];
                replaced_flag = true;
                continue;
            }

            if shifted_aa[cur_idx] != b {
                return false;
            }
        }

        shift_num == 0 || replaced_flag
    };

    (0..n).any(|shift_num| is_ok(shift_num))
}
