use itertools::join;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let mut left_dp: Vec<Option<usize>> = vec![None; n];
    left_dp[0] = Some(0);

    for city in 1..n {
        for dist in 1..=m.min(city) {
            if ss[city - dist][dist - 1] == '0' {
                continue;
            }

            if let Some(prev_cost) = left_dp[city - dist] {
                let cur_cost = &mut left_dp[city];
                let candidate_cost = prev_cost + 1;

                if cur_cost.is_none() || candidate_cost < cur_cost.unwrap() {
                    *cur_cost = Some(candidate_cost);
                }
            }
        }
    }

    let mut right_dp: Vec<Option<usize>> = vec![None; n];
    right_dp[n - 1] = Some(0);

    for city in (0..(n - 1)).rev() {
        for dist in 1..=m.min(n - 1 - city) {
            if ss[city][dist - 1] == '0' {
                continue;
            }

            if let Some(prev_cost) = right_dp[city + dist] {
                let cur_cost = &mut right_dp[city];
                let candidate_cost = prev_cost + 1;

                if cur_cost.is_none() || candidate_cost < cur_cost.unwrap() {
                    *cur_cost = Some(candidate_cost);
                }
            }
        }
    }

    let ans = (1..(n - 1)).map(|k| {
        let mut min_cost = None;

        for i in 1..m {
            let left = if i <= k {
                k - i
            } else {
                continue;
            };

            let left_cost = if let Some(left_cost) = left_dp[left] {
                left_cost
            } else {
                continue;
            };

            for j in 1..m {
                let right = if k + j < n {
                    k + j
                } else {
                    continue;
                };

                let right_cost = if let Some(right_cost) = right_dp[right] {
                    right_cost
                } else {
                    continue;
                };

                if right - left > m {
                    continue;
                }

                if ss[left][right - left - 1] == '0' {
                    continue;
                }

                let cost = left_cost + right_cost + 1;

                if min_cost.is_none() || cost < min_cost.unwrap() {
                    min_cost = Some(cost);
                }
            }
        }

        if let Some(min_cost) = min_cost {
            min_cost as i64
        } else {
            -1
        }
    });

    println!("{}", join(ans, " "));
}
