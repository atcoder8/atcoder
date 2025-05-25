use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        xyz: [(usize, usize, usize); t],
    }

    println!(
        "{}",
        xyz.iter()
            .map(|&(x, y, z)| if solve(x, y, z) { "Yes" } else { "No" })
            .join("\n")
    );
}

fn solve(x: usize, y: usize, z: usize) -> bool {
    if z == 0 {
        2 * x >= y && y % 2 == 0
    } else {
        2 * x >= y && x >= z
    }
}

/// Module for random testing.
#[cfg(test)]
mod random_test {
    use itertools::{enumerate, Itertools};
    use rand::{rngs::ThreadRng, Rng};

    use crate::solve;

    /// Input data type.
    type Input = (usize, usize, usize);

    /// Output data type.
    type Output = bool;

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
        std::array::from_fn(|_| rng.gen_range(0..=5)).into()
    }

    /// Returns the expected answer.
    pub fn expected(input: Input) -> Output {
        let (x, y, z) = input;

        let mut stack: Vec<(Vec<usize>, [usize; 3])> = vec![(vec![], [x, y, z])];
        while let Some((seq, rems)) = stack.pop() {
            if rems == [0; 3] {
                if seq
                    .iter()
                    .circular_tuple_windows()
                    .all(|(&r1, &r2, &r3)| [r1, r3].iter().filter(|&&r| r < r2).count() == r2)
                {
                    return true;
                }

                continue;
            }

            for (v, rem) in enumerate(rems) {
                if rem != 0 {
                    let mut next_seq = seq.clone();
                    next_seq.push(v);
                    let mut next_rems = rems;
                    next_rems[v] -= 1;
                    stack.push((next_seq, next_rems));
                }
            }
        }

        false
    }

    /// Solution to be tested.
    pub fn actual(input: Input) -> Output {
        let (x, y, z) = input;

        solve(x, y, z)
    }
}
