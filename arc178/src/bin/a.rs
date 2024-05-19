use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; m],
    }

    match solve(n, aa) {
        Some(ans) => println!("{}", ans.iter().join(" ")),
        None => println!("-1"),
    }
}

fn solve(n: usize, aa: Vec<usize>) -> Option<Vec<usize>> {
    if aa.contains(&1) || aa.contains(&n) {
        return None;
    }

    let mut included = vec![false; n + 1];
    for &a in &aa {
        included[a] = true;
    }

    let mut rem: BTreeSet<usize> = (1..=n).collect();

    let mut ans = vec![0; n];
    for i in 1..=n {
        let selected = if included[i] {
            *rem.range(i + 1..).next().unwrap()
        } else {
            *rem.first().unwrap()
        };

        ans[i - 1] = selected;
        rem.remove(&selected);
    }

    Some(ans)
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

    use crate::solve;

    /// Input data type.
    type Input = (usize, Vec<usize>);

    /// Output data type.
    type Output = Option<Vec<usize>>;

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
        let n = rng.gen_range(1..=6);
        let m = rng.gen_range(1..=n);
        let mut aa = (1..=n).collect_vec();
        aa.shuffle(rng);
        aa.truncate(m);

        (n, aa)
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let (n, aa) = input;

        let is_ok = |perm: &[usize]| {
            for &a in &aa {
                if perm.windows(a).any(|window| window.iter().all(|&p| p <= a)) {
                    return false;
                }
            }

            true
        };

        (1..=n).permutations(n).find(|perm| is_ok(perm))
    }

    /// Test this program.
    pub fn actual(input: Input) -> Output {
        let (n, aa) = input;

        solve(n, aa)
    }
}
