use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    println!("{}", solve(&aa));
}

fn solve(aa: &[usize]) -> usize {
    let n = aa.len();

    let mut ans = 0_usize;
    let mut sum = 0_usize;
    let mut counts = vec![0_usize; n];
    for (i, &a) in enumerate(aa) {
        let cnt = &mut counts[a];
        ans += *cnt * (i - *cnt) + sum - *cnt * (*cnt).saturating_sub(1) / 2;
        sum += *cnt;
        *cnt += 1;
    }

    ans
}

/// Module for random testing.
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::{rngs::ThreadRng, Rng};

    use super::*;

    /// Input data type.
    type Input = Vec<usize>;

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
        let n = rng.gen_range(3..=8);
        (0..n).map(|_| rng.gen_range(0..n)).collect()
    }

    /// Returns the expected answer.
    pub fn expected(input: Input) -> Output {
        let aa = input;

        aa.iter()
            .combinations(3)
            .filter(|sub_aa| sub_aa.iter().unique().count() == 2)
            .count()
    }

    /// Solution to be tested.
    pub fn actual(input: Input) -> Output {
        let aa = input;

        solve(&aa)
    }
}
