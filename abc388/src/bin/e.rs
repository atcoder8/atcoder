use itertools::izip;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u64; n],
    }

    println!("{}", solve(aa));
}

fn solve(aa: Vec<u64>) -> usize {
    let n = aa.len();

    let is_ok = |k: usize| izip!(&aa[..k], &aa[n - k..]).all(|(&small, &large)| 2 * small <= large);

    let mut ok = 0_usize;
    let mut ng = n / 2 + 1;

    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;

        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ok
}

/// Module for random testing.
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::{rngs::ThreadRng, Rng};

    use crate::solve;

    /// Input data type.
    type Input = Vec<u64>;

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
        let n = rng.gen_range(1_usize..=7);
        (0..n)
            .map(|_| rng.gen_range(1..=8))
            .sorted_unstable()
            .collect()
    }

    /// Returns the expected answer.
    pub fn expected(input: Input) -> Output {
        let aa = input;
        let n = aa.len();

        let count = |perm: &[u64]| {
            perm.chunks_exact(2)
                .filter(|chunk| 2 * chunk[0] <= chunk[1])
                .count()
        };

        aa.iter()
            .cloned()
            .permutations(n)
            .map(|perm| count(&perm))
            .max()
            .unwrap()
    }

    /// Solution to be tested.
    pub fn actual(input: Input) -> Output {
        let aa = input;
        solve(aa)
    }
}
