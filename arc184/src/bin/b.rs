use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!("{}", solve(n));
}

fn calc_coprime(max: usize) -> usize {
    max - (max / 2 + max / 3) + max / 6
}

fn calc_lengths(divided: usize) -> Vec<u32> {
    (0_u32..)
        .take_while(|exp2| divided >> exp2 != 0)
        .map(|exp2| (divided >> exp2).ilog(3) + 1)
        .chain([0])
        .collect()
}

fn calc_min_cost(lengths: &[u32]) -> usize {
    let mut dp: Vec<Option<usize>> = vec![None; 1 << lengths[0]];
    dp[0] = Some(0);
    for (&cur_length, &next_length) in lengths.iter().tuple_windows() {
        let mut next_dp: Vec<Option<usize>> = vec![None; 1 << next_length];
        let cur_mask = (1_usize << cur_length) - 1;
        let next_mask = (1_usize << next_length) - 1;
        for pos_bits in 0..1 << cur_length {
            let from_bits = ((1 << cur_length) - 1) ^ (pos_bits | pos_bits << 1) & cur_mask;
            let Some(cost) = dp[from_bits] else {
                continue;
            };

            chmin_for_option(
                &mut next_dp[pos_bits & next_mask],
                cost + pos_bits.count_ones() as usize,
            );
        }

        dp = next_dp;

        for bits in (0..1 << next_length).rev() {
            let Some(cost) = dp[bits] else {
                continue;
            };

            for i in 0..next_length {
                if bits >> i & 1 == 1 {
                    chmin_for_option(&mut dp[bits ^ (1 << i)], cost);
                }
            }
        }
    }

    dp[0].unwrap()
}

fn solve(n: usize) -> usize {
    let mut grid_points = vec![];
    for exp2 in 0..=n.ilog2() {
        for exp3 in 0..=(n >> exp2).ilog(3) {
            grid_points.push(3_usize.pow(exp3) << exp2);
        }
    }
    grid_points.sort_unstable();

    let mut boundaries = vec![1];
    while let Some(grid_point) = grid_points.pop() {
        let boundary = n / grid_point + 1;
        boundaries.push(boundary);

        while grid_points
            .last()
            .is_some_and(|grid_point| grid_point * boundary > n)
        {
            grid_points.pop();
        }
    }

    boundaries
        .iter()
        .tuple_windows()
        .map(|(&left, &right)| {
            let count = calc_coprime(right - 1) - calc_coprime(left - 1);
            let lengths = calc_lengths(n / left);
            count * calc_min_cost(&lengths)
        })
        .sum()
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}

/// Module for random testing.
#[cfg(test)]
mod random_test {
    use itertools::{enumerate, Itertools};
    use rand::{rngs::ThreadRng, Rng};

    use super::*;

    /// Input data type.
    type Input = usize;

    /// Output data type.
    type Output = usize;

    /// Performs the specified number of tests.
    #[test]
    fn test() {
        /// This value specifies the number of tests.
        const NUMBER_OF_TESTS: usize = 1000;

        let mut rng = rand::thread_rng();

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator(&mut rng);
            let expected_output = expected(input.clone());
            let actual_output = actual(input.clone());

            // If an unexpected answer is returned, panic is triggered.
            assert_eq!(
                expected_output, actual_output,
                "
Unexpected answer was returned in test case #{}.

[Input]
{:?}

[Expected output]
{:?}

[Actual output]
{:?}
",
                test_case_index, input, expected_output, actual_output
            );
        }
    }

    /// Generates a test case.
    pub fn generator(rng: &mut ThreadRng) -> Input {
        rng.gen_range(1..=10_usize.pow(4))
    }

    /// Returns the expected answer.
    pub fn expected(input: Input) -> Output {
        let n = input;

        let calc_min_cost = |u: usize| {
            let divided = n / u;

            let lengths = (0_u32..)
                .take_while(|exp2| divided >> exp2 != 0)
                .map(|exp2| (divided >> exp2).ilog(3) + 1)
                .chain([0])
                .collect_vec();
            let mut dp: Vec<Option<usize>> = vec![None; 1 << lengths[0]];
            dp[0] = Some(0);
            for (&cur_length, &next_length) in lengths.iter().tuple_windows() {
                let mut next_dp: Vec<Option<usize>> = vec![None; 1 << next_length];
                let filled_bits = (1_usize << cur_length) - 1;
                let mask = (1_usize << next_length) - 1;

                for (from_bits, &cost) in enumerate(&dp) {
                    let Some(cost) = cost else {
                        continue;
                    };

                    for pos_bits in 0..1 << cur_length {
                        let union_bits = from_bits | pos_bits | pos_bits << 1;
                        if union_bits & filled_bits != filled_bits {
                            continue;
                        }

                        chmin_for_option(
                            &mut next_dp[pos_bits & mask],
                            cost + pos_bits.count_ones() as usize,
                        );
                    }
                }

                dp = next_dp;
            }

            dp[0].unwrap()
        };

        (1..=n)
            .step_by(6)
            .chain((5..=n).step_by(6))
            .map(calc_min_cost)
            .sum()
    }

    /// Solution to be tested.
    pub fn actual(input: Input) -> Output {
        let n = input;

        super::solve(n)
    }
}
