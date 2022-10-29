// unfinished

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let mut continuous: Vec<Option<usize>> = vec![None; m + 1];
    let mut interrupted: Vec<Option<usize>> = vec![None; m + 1];
    interrupted[0] = Some(0);

    for &a in &aa {
        let next_interrupted = continuous
            .iter()
            .zip(interrupted.iter())
            .map(|(&cnt1, &cnt2)| match (cnt1, cnt2) {
                (None, None) => None,
                (None, Some(cnt2)) => Some(cnt2),
                (Some(cnt1), None) => Some(cnt1),
                (Some(cnt1), Some(cnt2)) => Some(cnt1.min(cnt2)),
            })
            .collect_vec();

        let mut next_continuous = vec![None; m + 1];

        if a > m {
            continue;
        }

        for i in a..=m {
            if let Some(cnt) = continuous[i - a] {
                if next_continuous[i].is_none() || cnt < next_continuous[i].unwrap() {
                    next_continuous[i] = Some(cnt);
                }
            }

            if let Some(cnt) = interrupted[i - a] {
                if next_continuous[i].is_none() || cnt + 1 < next_continuous[i].unwrap() {
                    next_continuous[i] = Some(cnt + 1);
                }
            }
        }

        continuous = next_continuous;
        interrupted = next_interrupted;
    }

    for i in 1..=m {
        let ans = match (continuous[i], interrupted[i]) {
            (None, None) => None,
            (None, Some(cnt2)) => Some(cnt2 + 1),
            (Some(cnt1), None) => Some(cnt1),
            (Some(cnt1), Some(cnt2)) => Some(cnt1.min(cnt2 + 1)),
        };

        if let Some(ans) = ans {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}
