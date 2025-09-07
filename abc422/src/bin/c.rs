use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        abc: [(usize, usize, usize); t],
    }

    let output = abc.iter().map(|&(a, b, c)| solve(a, b, c)).join("\n");
    println!("{output}");
}

fn solve(mut a: usize, mut b: usize, mut c: usize) -> usize {
    let num_abc = a.min(b).min(c);
    [&mut a, &mut b, &mut c]
        .into_iter()
        .for_each(|v| *v -= num_abc);
    if a < c {
        std::mem::swap(&mut a, &mut c);
    }
    let num_aac = (a - c).min(c);
    a -= 2 * num_aac;
    c -= num_aac;
    let num_pairs = a.min(c) / 3;
    a -= 3 * num_pairs;
    c -= 3 * num_pairs;

    num_abc + num_aac + 2 * num_pairs + (a >= 2 && c >= 1) as usize
}

/// Module for random testing.
#[cfg(test)]
mod random_test {
    use rand::{rngs::ThreadRng, Rng};

    use crate::solve;

    /// Input data type.
    type Input = (usize, usize, usize);

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
        let a = rng.gen_range(0..=10);
        let b = rng.gen_range(0..=10);
        let c = rng.gen_range(0..=10);
        (a, b, c)
    }

    /// Returns the expected answer.
    pub fn expected(input: Input) -> Output {
        let mut max_num_contests = 0_usize;

        let (a, b, c) = input;
        for num_aac in 0..=(a / 2).min(c) {
            let rem_a = a - 2 * num_aac;
            let rem_c = c - num_aac;
            for num_abc in 0..=rem_a.min(b).min(rem_c) {
                let rem_a = rem_a - num_abc;
                let rem_c = rem_c - num_abc;
                let num_acc = rem_a.min(rem_c / 2);
                max_num_contests = max_num_contests.max(num_aac + num_abc + num_acc);
            }
        }

        max_num_contests
    }

    /// Solution to be tested.
    pub fn actual(input: Input) -> Output {
        let (a, b, c) = input;
        solve(a, b, c)
    }
}
