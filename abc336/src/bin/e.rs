use itertools::Itertools;
use proconio::input;

const MAX_DIGIT_SUM: usize = 9 * 14;

fn main() {
    input! {
        n: usize,
    }

    let digits = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let mut ans = 0;
    for target_digit_sum in 1..=MAX_DIGIT_SUM {
        // less[i][j] + equal[i][j]: Nのprefix以下の非負整数であって、桁和がiで数値をtarget_digit_sumで割るとjになるものの個数
        let mut less = vec![vec![0_usize; target_digit_sum]; target_digit_sum + 1];
        let mut equal = vec![vec![0_usize; target_digit_sum]; target_digit_sum + 1];
        equal[0][0] = 1;

        for &d in &digits {
            let mut next_less = vec![vec![0; target_digit_sum]; target_digit_sum + 1];
            let mut next_equal = vec![vec![0; target_digit_sum]; target_digit_sum + 1];

            // less -> next_less
            for add in 0..10 {
                for digit_sum in add..=target_digit_sum {
                    for prev_rem in 0..target_digit_sum {
                        next_less[digit_sum][(10 * prev_rem + add) % target_digit_sum] +=
                            less[digit_sum - add][prev_rem];
                    }
                }
            }

            // equal -> next_less
            for add in 0..d {
                for digit_sum in add..=target_digit_sum {
                    for prev_rem in 0..target_digit_sum {
                        next_less[digit_sum][(10 * prev_rem + add) % target_digit_sum] +=
                            equal[digit_sum - add][prev_rem];
                    }
                }
            }

            // equal -> next_equal
            for digit_sum in d..=target_digit_sum {
                for prev_rem in 0..target_digit_sum {
                    next_equal[digit_sum][(10 * prev_rem + d) % target_digit_sum] +=
                        equal[digit_sum - d][prev_rem];
                }
            }

            less = next_less;
            equal = next_equal;
        }

        ans += less[target_digit_sum][0] + equal[target_digit_sum][0];
    }

    println!("{}", ans);
}
