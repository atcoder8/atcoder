// unfinished

use itertools::join;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut pp: [usize; n],
    }

    let mut left = 0;
    let mut right = 0;
    let mut max_right = None;
    let mut max_right_left = None;
    for (i, &p) in pp.iter().enumerate() {
        right = left + pp[left..right].upper_bound(&p);

        if right == left {
            left = i;
            right = i + 1;
        }

        if i - left + 1 == k && (max_right.is_none() || right > max_right.unwrap()) {
            max_right = Some(right);
            max_right_left = Some(left);

            left += 1;
        }
    }

    let left = max_right_left.unwrap_or(n - k);
    pp[left..(left + k)].sort_unstable();

    println!("{}", join(pp, " "));
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::{seq::SliceRandom, Rng};
    use superslice::Ext;

    /// Input data type.
    type Input = (usize, Vec<usize>);

    /// Output data type.
    type Output = Vec<usize>;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let expected_output = expected(input.clone());
            let actual_output = solve(input.clone());

            assert_eq!(
                expected_output, actual_output,
                "
Wrong Answer on Test #{}

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

    /// Generate test cases.
    pub fn generator() -> Input {
        let mut rng = rand::thread_rng();

        let n = rng.gen_range(1..=5);
        let k = rng.gen_range(1..=n);
        let mut pp = (1..=n).collect_vec();
        pp.shuffle(&mut rng);

        (k, pp)
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let (k, pp) = input;
        let n = pp.len();

        (0..=(n - k))
            .map(|i| {
                let mut qq = pp.clone();
                qq[i..(i + k)].sort_unstable();

                qq
            })
            .max()
            .unwrap()
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let (k, mut pp) = input;

        let n = pp.len();
        let mut left = 0;
        let mut right = 0;
        let mut max_right = None;
        let mut max_right_left = None;
        for (i, &p) in pp.iter().enumerate() {
            let right_pos = left + pp[left..right].upper_bound(&p);
            if right_pos == i + 1 {
                right = right_pos;
            } else {
                right = right_pos.min(right);
            }

            if right == left {
                left = i;
                right = i + 1;
            }

            if i - left + 1 == k && (max_right.is_none() || right > max_right.unwrap()) {
                max_right = Some(right);
                max_right_left = Some(left);

                left += 1;
            }
        }

        let left = max_right_left.unwrap_or(n - k);
        pp[left..(left + k)].sort_unstable();

        pp
    }
}
