use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; m],
        ss: [Chars; n],
    }

    let mut scores = (1..=n).collect_vec();
    for i in 0..n {
        for j in 0..m {
            if ss[i][j] == 'o' {
                scores[i] += aa[j];
            }
        }
    }
    let max_score = *scores.iter().max().unwrap();

    let mut ans = vec![0; n];
    for i in 0..n {
        let mut unsolved = vec![];
        for j in 0..m {
            if ss[i][j] == 'x' {
                unsolved.push(aa[j]);
            }
        }
        unsolved.sort_unstable_by_key(|&x| Reverse(x));
        let mut rem = max_score - scores[i];
        let mut cnt = 0;
        for x in unsolved {
            if rem == 0 {
                break;
            }

            rem = rem.saturating_sub(x);
            cnt += 1;
        }

        ans[i] = cnt;
    }

    println!("{}", ans.iter().join("\n"));
}
