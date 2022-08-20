use itertools::join;

fn main() {
    println!("{}", join(solve().iter().map(|&x| x + 1), " "));
}

fn solve() -> Vec<usize> {
    let (n, k) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let pp = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>()
    };

    if k == 0 {
        return pp;
    }

    let mut inv_pp = vec![0; n];
    pp.iter().enumerate().for_each(|(i, &p)| inv_pp[p] = i);

    let smallest_front = *pp.iter().take(k + 1).min().unwrap();
    let smallest_back = *pp.iter().skip(n - k).min().unwrap();

    let ans_without_rotation = {
        let smallest_idx = inv_pp[smallest_front];

        let mut ans = vec![];
        let mut cnt = smallest_idx;

        for &p in pp.iter().skip(smallest_idx) {
            while !ans.is_empty() && cnt < k && *ans.last().unwrap() > p {
                ans.pop();
                cnt += 1;
            }
            ans.push(p);
        }

        ans.resize(ans.len() - (k - cnt), 0);

        ans
    };

    let ans_with_rotation = {
        let smallest_idx = inv_pp[smallest_back];

        let mut cnt = n - smallest_idx;

        let mut front = vec![];
        for &p in pp.iter().skip(smallest_idx) {
            while !front.is_empty() && *front.last().unwrap() > p {
                front.pop();
            }
            front.push(p);
        }

        let mut back = vec![];
        for &p in pp.iter().take(smallest_idx) {
            while !back.is_empty() && cnt < k && *back.last().unwrap() > p {
                back.pop();
                cnt += 1;
            }
            back.push(p);
        }

        while !front.is_empty() && front.last().unwrap() > back.first().unwrap() {
            front.pop();
        }

        front.append(&mut back);

        front.resize(front.len() - (k - cnt), 0);

        front
    };

    ans_without_rotation.min(ans_with_rotation)
}

/// Module for testing
#[cfg(test)]
mod random_test {
    use itertools::Itertools;
    use rand::{prelude::SliceRandom, Rng};

    /// Input data type
    type Input = (usize, usize, Vec<usize>);

    /// Output data type
    type Output = Vec<usize>;

    /// Perform the specified number of tests.
    #[test]
    fn test() {
        const NUMBER_OF_TESTS: usize = 1000;

        for test_case_index in 0..NUMBER_OF_TESTS {
            let input = generator();
            let jury_output = jury(input.clone());
            let solve_output = solve(input.clone());

            assert_eq!(
                jury_output, solve_output,
                "
Wrong Answer on Test #{}

[Input]
{:?}

[Output(Jury)]
{:?}

[Output(Solve)]
{:?}
",
                test_case_index, input, jury_output, solve_output
            );
        }
    }

    /// Generate test cases.
    pub fn generator() -> Input {
        let mut rng = rand::thread_rng();

        let n = rng.gen_range(1, 11);
        let k = rng.gen_range(0, n);

        let mut pp: Vec<usize> = (0..n).collect();
        pp.shuffle(&mut rng);

        (n, k, pp)
    }

    /// Returns the correct answer.
    pub fn jury(input: Input) -> Output {
        let (n, k, pp) = input;

        let mut ans = pp.clone();

        for delete_num in 0..(k + 1) {
            for rem_terms in pp.clone().into_iter().combinations(n - delete_num) {
                let boundary_index = n - k;
                let after_pp = [
                    rem_terms[boundary_index..].to_vec(),
                    rem_terms[..boundary_index].to_vec(),
                ]
                .concat();
                ans = ans.min(after_pp);
            }
        }

        ans
    }

    /// Test this program.
    pub fn solve(input: Input) -> Output {
        let (n, k, pp) = input;

        if k == 0 {
            return pp;
        }

        let mut inv_pp = vec![0; n];
        pp.iter().enumerate().for_each(|(i, &p)| inv_pp[p] = i);

        let smallest_front = *pp.iter().take(k + 1).min().unwrap();
        let smallest_back = *pp.iter().skip(n - k).min().unwrap();

        let ans_without_rotation = {
            let smallest_idx = inv_pp[smallest_front];

            let mut ans = vec![];
            let mut cnt = smallest_idx;

            for &p in pp.iter().skip(smallest_idx) {
                while !ans.is_empty() && cnt < k && *ans.last().unwrap() > p {
                    ans.pop();
                    cnt += 1;
                }
                ans.push(p);
            }

            ans.resize(ans.len() - (k - cnt), 0);

            ans
        };

        let ans_with_rotation = {
            let smallest_idx = inv_pp[smallest_back];

            let mut cnt = n - smallest_idx;

            let mut front = vec![];
            for &p in pp.iter().skip(smallest_idx) {
                while !front.is_empty() && *front.last().unwrap() > p {
                    front.pop();
                }
                front.push(p);
            }

            let mut back = vec![];
            for &p in pp.iter().take(smallest_idx) {
                while !back.is_empty() && cnt < k && *back.last().unwrap() > p {
                    back.pop();
                    cnt += 1;
                }
                back.push(p);
            }

            while !front.is_empty() && front.last().unwrap() > back.first().unwrap() {
                front.pop();
            }

            front.append(&mut back);

            front.resize(front.len() - (k - cnt), 0);

            front
        };

        ans_without_rotation.min(ans_with_rotation)
    }
}
