use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
    }

    println!("{}", solve(n, k).iter().join(" "));
}

fn solve(n: usize, k: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1; k];
    }

    let half_n = (n + 1) / 2;

    if n % 2 == 0 {
        let mut seq = vec![half_n];
        for i in (1..=n).rev() {
            seq.extend(vec![i; k - (i == half_n) as usize]);
        }

        seq
    } else {
        let mut seq = vec![half_n; k];
        seq.push(half_n - 1);
        for i in (1..=n).rev() {
            if i == half_n {
                continue;
            }

            seq.extend(vec![i; k - (i == half_n - 1) as usize]);
        }

        seq
    }
}

/// If there is a next permutation of `seq` with respect to the lexicographic order, replace `seq` with it (return value is `true`).
/// Otherwise (i.e., if `seq` is already in descending order), it reverts to ascending order (return value is `false`).
pub fn next_permutation<T>(seq: &mut [T]) -> bool
where
    T: Ord,
{
    // If the length of `seq` is 0 or 1, the next permutation does not exist.
    if seq.len() <= 1 {
        return false;
    }

    // Find the maximum value of `i` such that `seq[i] < seq[i + 1]`.
    // If no such `i` exists, `seq` has already been sorted in descending order.
    let Some(i) = (0..seq.len() - 1).rev().find(|&i| seq[i] < seq[i + 1]) else {
        seq.reverse();
        return false;
    };

    // Find the largest `j` that satisfies `i < j` and `seq[i] < seq[j]`, and exchange `seq[i]` and `seq[j]`.
    let j = (i + 1..seq.len()).rev().find(|&j| seq[i] < seq[j]).unwrap();
    seq.swap(i, j);

    // Sort elements after the `i`-th in ascending order to minimize the increase with respect to lexicographic order.
    seq[i + 1..].reverse();

    true
}

/// Module for random testing.
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use permutohedron::factorial;
    use rand::{rngs::ThreadRng, Rng};

    use crate::{next_permutation, solve};

    /// Input data type.
    type Input = (usize, usize);

    /// Output data type.
    type Output = Vec<usize>;

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
        let n = rng.gen_range(1..=4);
        let k = rng.gen_range(1..=3);

        (n, k)
    }

    /// Returns the expected answer.
    pub fn expected(input: Input) -> Output {
        let (n, k) = input;

        let mut seq = (1..=n)
            .flat_map(|i| std::iter::repeat(i).take(k))
            .collect_vec();

        let num_perms = factorial(n * k) / factorial(k).pow(n as u32);
        for _ in 0..(num_perms - 1) / 2 {
            assert!(next_permutation(&mut seq));
        }

        seq
    }

    /// Solution to be tested.
    pub fn actual(input: Input) -> Output {
        let (n, k) = input;

        solve(n, k)
    }
}
