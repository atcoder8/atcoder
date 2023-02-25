use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, k): (usize, usize),
        s: Chars,
    }

    let mut x_sections = vec![];

    let mut left = 0;

    while left < n {
        let mut right = left;
        while right < n && s[right] == 'X' {
            right += 1;
        }

        let size = right - left;

        if size != 0 {
            if left == 0 && right == n {
                x_sections.push((n + 1, size, size - 1));
            } else if left == 0 || right == n {
                x_sections.push((n + 1, size, size));
            } else {
                x_sections.push((size, size, size + 1));
            };
        }

        left = right + 1;
    }

    let mut y_sections = vec![];

    let mut left = 0;

    while left < n {
        let mut right = left;
        while right < n && s[right] == 'Y' {
            right += 1;
        }

        let size = right - left;

        if size != 0 {
            if left == 0 && right == n {
                y_sections.push((n + 1, size, size - 1));
            } else if left == 0 || right == n {
                y_sections.push((n + 1, size, size));
            } else {
                y_sections.push((size, size, size + 1));
            };
        }

        left = right + 1;
    }

    x_sections.sort_by_key(|x| Reverse(x.0));

    let mut ans = (0..(n - 1))
        .filter(|&i| s[i] == 'Y' && s[i + 1] == 'Y')
        .count();
    let mut rem = k;

    while let Some((_, size, potential)) = x_sections.pop() {
        if rem == 0 {
            break;
        }

        if size <= rem {
            ans += potential;
            rem -= size;
        } else {
            if size == n {
                ans += rem - 1;
            } else {
                ans += rem;
                rem = 0;
            }
        }
    }

    if rem > 0 {
        y_sections.sort_by_key(|x| x.0);

        while let Some((v, size, potential)) = y_sections.pop() {
            if rem == 0 {
                break;
            }

            if size <= rem {
                ans -= potential;
                rem -= size;
            } else {
                if v != size {
                    ans -= rem;
                } else {
                    ans -= rem + 1;
                }
                rem = 0;
            }
        }
    }

    println!("{}", ans);
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use std::cmp::Reverse;

    use itertools::Itertools;
    use rand::Rng;

    /// Input data type.
    type Input = (Vec<char>, usize);

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

        let n = rng.gen_range(1, 10);
        let k = rng.gen_range(0, n + 1);

        let s = (0..n)
            .map(|_| if rng.gen_bool(0.5) { 'X' } else { 'Y' })
            .collect_vec();

        (s, k)
    }

    /// Return the expected answer.
    pub fn expected(input: Input) -> Output {
        let (s, k) = input;
        let n = s.len();

        let seq = (0..n).collect_vec();

        seq.iter()
            .combinations(k)
            .map(|comb| {
                let mut t = s.clone();

                for &idx in comb {
                    t[idx] = if t[idx] == 'X' { 'Y' } else { 'X' };
                }

                (0..(n - 1))
                    .filter(|&i| t[i] == 'Y' && t[i + 1] == 'Y')
                    .count()
            })
            .max()
            .unwrap()
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let (s, k) = input;
        let n = s.len();

    let mut x_sections = vec![];

    let mut left = 0;

    while left < n {
        let mut right = left;
        while right < n && s[right] == 'X' {
            right += 1;
        }

        let size = right - left;

        if size != 0 {
            if left == 0 && right == n {
                x_sections.push((n + 1, size, size - 1));
            } else if left == 0 || right == n {
                x_sections.push((n + 1, size, size));
            } else {
                x_sections.push((size, size, size + 1));
            };
        }

        left = right + 1;
    }

    let mut y_sections = vec![];

    let mut left = 0;

    while left < n {
        let mut right = left;
        while right < n && s[right] == 'Y' {
            right += 1;
        }

        let size = right - left;

        if size != 0 {
            if left == 0 && right == n {
                y_sections.push((n + 1, size, size - 1));
            } else if left == 0 || right == n {
                y_sections.push((n + 1, size, size));
            } else {
                y_sections.push((size, size, size + 1));
            };
        }

        left = right + 1;
    }

    x_sections.sort_by_key(|x| Reverse(x.0));

    let mut ans = (0..(n - 1))
        .filter(|&i| s[i] == 'Y' && s[i + 1] == 'Y')
        .count();
    let mut rem = k;

    while let Some((_, size, potential)) = x_sections.pop() {
        if rem == 0 {
            break;
        }

        if size <= rem {
            ans += potential;
            rem -= size;
        } else {
            if size == n {
                ans += rem - 1;
            } else {
                ans += rem;
                rem = 0;
            }
        }
    }

    if rem > 0 {
        y_sections.sort_by_key(|x| x.0);

        while let Some((v, size, potential)) = y_sections.pop() {
            if rem == 0 {
                break;
            }

            if size <= rem {
                ans -= potential;
                rem -= size;
            } else {
                if v != size {
                    ans -= rem;
                } else {
                    ans -= rem + 1;
                }
                rem = 0;
            }
        }
    }

        ans
    }
}
