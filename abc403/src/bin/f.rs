use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![vec![]; n + 1];
    for i in [1, 11, 111, 1111] {
        if i <= n {
            dp[i] = vec![i.to_string(), i.to_string()];
        }
    }

    for i in 1..=n {
        if [1, 11, 111, 1111].contains(&i) {
            continue;
        }

        let shortest_add_formula = iproduct!(1..=i - 1, [false, true], [false, true])
            .map(|(left_value, left_multi, right_multi)| {
                let right_value = i - left_value;
                let left_formula = &dp[left_value][left_multi as usize];
                let right_formula = &dp[right_value][right_multi as usize];
                format!("{}+{}", left_formula, right_formula)
            })
            .min_by_key(|formula| formula.len())
            .unwrap();

        let mut shortest_mul_formula = format!("({})", shortest_add_formula);
        let divisors = find_divisors(i);
        for &left_value in &divisors {
            if left_value == 1 || left_value == i {
                continue;
            }

            let right_value = i / left_value;
            for (left_multi, right_multi) in iproduct!([false, true], [false, true]) {
                let left_formula = if left_multi {
                    format!("({})", dp[left_value][1])
                } else {
                    dp[left_value][0].clone()
                };

                let right_formula = if right_multi {
                    format!("({})", dp[right_value][1])
                } else {
                    dp[right_value][0].clone()
                };

                let cand_formula = format!("{}*{}", left_formula, right_formula);
                if cand_formula.len() < shortest_mul_formula.len() {
                    shortest_mul_formula = cand_formula;
                }
            }
        }

        dp[i] = vec![shortest_mul_formula, shortest_add_formula];
    }

    println!(
        "{}",
        std::cmp::min_by_key(&dp[n][0], &dp[n][1], |formula| formula.len())
    );
}

/// Creates a sequence consisting of the divisors of `n`.
pub fn find_divisors(n: usize) -> Vec<usize> {
    assert_ne!(n, 0, "`n` must be at least 1.");

    let mut divisors = vec![];

    for i in (1..).take_while(|&i| i <= n / i) {
        if n % i == 0 {
            divisors.push(i);

            if n / i != i {
                divisors.push(n / i);
            }
        }
    }

    divisors.sort_unstable();

    divisors
}
