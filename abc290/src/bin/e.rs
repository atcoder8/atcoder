// unfinished

use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use rand::Rng;

    /// Input data type.
    type Input = Vec<usize>;

    /// Output data type.
    type Output = usize;

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

        let n = rng.gen_range(1, 5);

        (0..n).map(|_| rng.gen_range(1, n + 1)).collect()
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let aa = input;
        let n = aa.len();

        let mut ans = 0;

        for l in 0..n {
            for r in (l + 1)..=n {
                let len = r - l;
                ans += (0..(len / 2))
                    .filter(|&i| aa[l + i] != aa[r - 1 - i])
                    .count();
            }
        }

        ans
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let aa = input;
        let n = aa.len();

        let mut sum = 0;
        let mut ans = 0;
        let mut counts = vec![0; n];

        for (i, &a) in aa.iter().enumerate() {
            ans += sum - counts[a - 1];
            let dup = (i + 1).min(n - 1 - i);
            counts[a - 1] += dup;
            sum += dup;
        }

        ans
    }
}
