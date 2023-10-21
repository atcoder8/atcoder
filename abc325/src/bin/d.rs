use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut td: [(usize, usize); n],
    }

    println!("{}", solve(td));
}

fn solve(mut td: Vec<(usize, usize)>) -> usize {
    td.sort_unstable_by_key(|x| x.0);

    let mut ans = 0;
    let mut heap = BinaryHeap::new();
    let mut charged_time = 0;
    for (key, group) in &td.into_iter().group_by(|x| x.0) {
        while let Some(Reverse(finish)) = heap.pop() {
            if finish >= charged_time {
                ans += 1;
                charged_time += 1;

                if charged_time == key {
                    break;
                }
            }
        }

        heap.extend(group.map(|(t, d)| Reverse(t + d)));

        charged_time = key;
    }

    while let Some(Reverse(finish)) = heap.pop() {
        if finish >= charged_time {
            ans += 1;
            charged_time += 1;
        }
    }

    ans
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::Rng;

    /// Input data type.
    type Input = Vec<(usize, usize)>;

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

        let n = rng.gen_range(1..=5);
        let td = (0..n)
            .map(|_| (rng.gen_range(1..=5), rng.gen_range(1..=5)))
            .collect();

        td
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let td = input;
        let n = td.len();

        let mut ans = 0;
        for perm in (0..n).permutations(n) {
            let mut cnt = 0;
            let mut prev = 0;
            for &i in &perm {
                let (t, d) = td[i];
                if t + d > prev {
                    cnt += 1;
                    prev = (prev + 1).max(t);
                }
            }

            ans = ans.max(cnt);
        }

        ans
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let td = input;

        super::solve(td)
    }
}
