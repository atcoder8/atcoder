use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }

    println!("{}", solve(lr));
}

fn solve(lr: Vec<(usize, usize)>) -> usize {
    let events = lr
        .iter()
        .flat_map(|&(l, r)| [(l, false), (r, true)])
        .sorted_unstable();

    let mut ans = 0;
    let mut cnt = 0;

    for (_, out) in events {
        if out {
            cnt -= 1;
            continue;
        }

        ans += cnt;
        cnt += 1;
    }

    ans
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::{rngs::ThreadRng, Rng};

    use crate::solve;

    /// Input data type.
    type Input = Vec<(usize, usize)>;

    /// Output data type.
    type Output = usize;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        let mut rng = rand::thread_rng();

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator(&mut rng);
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
    pub fn generator(rng: &mut ThreadRng) -> Input {
        let n = rng.gen_range(1..=10);
        let lr = (0..n)
            .map(|_| {
                let l = rng.gen_range(0..10);
                let r = rng.gen_range(l + 1..=10);

                (l, r)
            })
            .collect_vec();

        lr
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let lr = input;

        lr.iter()
            .tuple_combinations()
            .filter(|(&(l1, r1), &(l2, r2))| l1 <= r2 && l2 <= r1)
            .count()
    }

    /// Test this program.
    pub fn actual(input: Input) -> Output {
        let lr = input;

        solve(lr)
    }
}
