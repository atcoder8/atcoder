use itertools::{join, Itertools};
use num_integer::Roots;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [usize; n],
        lr: [(Usize1, usize); q],
    }

    let max_a = *aa.iter().max().unwrap();

    let b = n / q.sqrt() + 1;

    let mut ilr = lr.iter().cloned().enumerate().collect_vec();
    ilr.sort_unstable_by_key(|&(_, (l, r))| (l / b, r));

    let mut counts = vec![0_i64; max_a + 1];
    let mut comb_cnt = 0_i64;
    let mut left = 0;
    let mut right = 0;

    let mut ans = vec![0; q];

    for &(i, (l, r)) in &ilr {
        if left <= l {
            for i in left..l {
                let cnt = &mut counts[aa[i]];
                comb_cnt -= (*cnt - 1) * (*cnt - 2) / 2;
                *cnt -= 1;
            }
        } else {
            for i in l..left {
                let cnt = &mut counts[aa[i]];
                comb_cnt += *cnt * (*cnt - 1) / 2;
                *cnt += 1;
            }
        }
        left = l;

        if right <= r {
            for i in right..r {
                let cnt = &mut counts[aa[i]];
                comb_cnt += *cnt * (*cnt - 1) / 2;
                *cnt += 1;
            }
        } else {
            for i in r..right {
                let cnt = &mut counts[aa[i]];
                comb_cnt -= (*cnt - 1) * (*cnt - 2) / 2;
                *cnt -= 1;
            }
        }
        right = r;

        ans[i] = comb_cnt;
    }

    println!("{}", join(ans, "\n"));
}
