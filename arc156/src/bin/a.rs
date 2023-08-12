use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        if let Some(ans) = solve() {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        s: Chars,
    }

    let front_num = s.iter().filter(|&&c| c == '1').count();
    if front_num % 2 == 1 {
        return None;
    }

    if n == 3 && s[1] == '1' {
        return None;
    }

    if s == vec!['0', '1', '1', '0'] {
        return Some(3);
    }

    if front_num == 2 && (0..(n - 1)).any(|i| s[i] == '1' && s[i + 1] == '1') {
        return Some(front_num / 2 + 1);
    }

    Some(front_num / 2)
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use std::collections::VecDeque;

    use im_rc::HashSet;
    use itertools::Itertools;
    use rand::Rng;

    /// Input data type
    type Input = Vec<char>;

    /// Output data type
    type Output = Option<usize>;

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

        let n = rng.gen_range(3_usize..10);
        let bit = rng.gen_range(0..(1_usize << n));
        let s = (0..n)
            .map(|i| (b'0' + ((bit >> i) & 1) as u8) as char)
            .collect_vec();

        s
    }

    /// Returns expected answer.
    pub fn expected(input: Input) -> Output {
        let s = input;
        let n = s.len();

        let init = s.iter().map(|&c| c == '1').collect_vec();

        let mut used: HashSet<Vec<bool>> = HashSet::new();
        used.insert(init.clone());

        let mut queue: VecDeque<(Vec<bool>, usize)> = VecDeque::from(vec![(init, 0)]);

        while let Some((cur, cnt)) = queue.pop_front() {
            if cur.iter().all(|&x| !x) {
                return Some(cnt);
            }

            for i in 0..(n - 2) {
                for j in (i + 2)..n {
                    let mut next = cur.clone();

                    next[i] = !next[i];
                    next[j] = !next[j];

                    if !used.contains(&next) {
                        queue.push_back((next.clone(), cnt + 1));
                        used.insert(next);
                    }
                }
            }
        }

        None
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let s = input;
        let n = s.len();

        let front_num = s.iter().filter(|&&c| c == '1').count();
        if front_num % 2 == 1 {
            return None;
        }

        if n == 3 && s[1] == '1' {
            return None;
        }

        if s == vec!['0', '1', '1', '0'] {
            return Some(3);
        }

        if front_num == 2 && (0..(n - 1)).any(|i| s[i] == '1' && s[i + 1] == '1') {
            return Some(front_num / 2 + 1);
        }

        Some(front_num / 2)
    }
}
