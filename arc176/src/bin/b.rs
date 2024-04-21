use itertools::Itertools;
use proconio::input;

const CYCLE: [usize; 4] = [2, 4, 8, 6];

fn main() {
    input! {
        t: usize,
    }

    let ans = (0..t)
        .map(|_| {
            input! {
                (n, m, k): (usize, usize, usize),
            }

            solve(n, m, k)
        })
        .join("\n");
    println!("{}", ans);
}

fn solve(n: usize, m: usize, k: usize) -> usize {
    if m == n + 1 && k == n {
        return 0;
    }

    if m > n {
        return CYCLE[(n - 1) % 4];
    }

    if m == k + 1 {
        return 0;
    }

    CYCLE[((n - k) % (m - k) + k - 1) % 4]
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use num::BigUint;
    use rand::Rng;

    use crate::solve;

    /// Input data type.
    type Input = (usize, usize, usize);

    /// Output data type.
    type Output = usize;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let expected_output = expected(input.clone());
            let actual_output = actual(input.clone());

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

        let n = rng.gen_range(1..=10);
        let m = rng.gen_range(2..=10);
        let k = rng.gen_range(1..m);

        (n, m, k)
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let (n, m, k) = input;

        let a = BigUint::from(2_u32).pow(n as u32);
        let b = BigUint::from(2_u32).pow(m as u32);
        let c = BigUint::from(2_u32).pow(k as u32);

        let rem = a % (b - c);
        rem.to_string()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize
    }

    /// Test this program.
    pub fn actual(input: Input) -> Output {
        let (n, m, k) = input;

        solve(n, m, k)
    }
}
