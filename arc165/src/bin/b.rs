use itertools::join;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut pp: [usize; n],
    }

    let ans = solve((k, pp));
    println!("{}", join(ans, " "));
}

fn solve(input: (usize, Vec<usize>)) -> Vec<usize> {
    let (k, mut pp) = input;
    let n = pp.len();

    let mut max_inc_len = 1;
    let mut inc_len = 1;
    for window in pp.windows(2) {
        if window[0] < window[1] {
            inc_len += 1;
            max_inc_len = max_inc_len.max(inc_len);
        } else {
            inc_len = 1;
        }
    }

    if max_inc_len >= k {
        return pp;
    }

    let mut acc_min = vec![0; k];
    acc_min[0] = pp[n - k];
    for i in 1..k {
        acc_min[i] = acc_min[i - 1].min(pp[n - k + i]);
    }

    let left = (((n - k).saturating_sub(k - 1))..(n - k))
        .rev()
        .take_while(|&i| pp[i] < pp[i + 1])
        .filter(|&i| pp[n - k - 1] < acc_min[i + k - 1 - (n - k)])
        .min();

    match left {
        Some(left) => pp[left..(left + k)].sort_unstable(),
        None => pp[(n - k)..].sort_unstable(),
    }

    pp
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::{seq::SliceRandom, Rng};

    /// Input data type.
    type Input = (usize, Vec<usize>);

    /// Output data type.
    type Output = Vec<usize>;

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

        let n = rng.gen_range(1..=100);
        let k = rng.gen_range(1..=n);
        let mut pp = (1..=n).collect_vec();
        pp.shuffle(&mut rng);

        (k, pp)
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let (k, pp) = input;
        let n = pp.len();

        (0..=(n - k))
            .map(|i| {
                let mut qq = pp.clone();
                qq[i..(i + k)].sort_unstable();

                qq
            })
            .max()
            .unwrap()
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        super::solve(input)
    }
}
