use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<usize> {
    input! {
        (h, w, k): (usize, usize, usize),
        ss: [Chars; h],
    }

    let calc_cost = |s: &[char]| {
        let n = s.len();

        let mut min_cost: Option<usize> = None;

        let mut left = 0;
        let mut right = 0;
        let mut cost = 0;
        while right < n {
            if right - left == k {
                cost -= (s[left] == '.') as usize;
                left += 1;
            }

            match s[right] {
                'o' => {}
                'x' => {
                    left = right + 1;
                    cost = 0;
                }
                '.' => {
                    cost += 1;
                }
                _ => unreachable!(),
            }

            right += 1;

            if right - left == k {
                update_cost(&mut min_cost, cost);
            }
        }

        min_cost
    };

    let mut ans: Option<usize> = None;

    for row in 0..h {
        if let Some(cand_cost) = calc_cost(&ss[row]) {
            update_cost(&mut ans, cand_cost);
        }
    }

    for col in 0..w {
        let s = (0..h).map(|row| ss[row][col]).collect_vec();
        if let Some(cand_cost) = calc_cost(&s) {
            update_cost(&mut ans, cand_cost);
        }
    }

    ans
}

/// Updates the minimum cost.
/// If `cost` is `None`, always updated to the candidate cost.
///
/// # Arguments
///
/// * `cost` - Reference variable for the cost to be updated.
/// * `cand_cost` - Candidate cost to update.
pub fn update_cost<T>(cost: &mut Option<T>, cand_cost: T) -> bool
where
    T: PartialOrd,
{
    if cost.as_ref().is_some_and(|cost| cost <= &cand_cost) {
        return false;
    }

    *cost = Some(cand_cost);

    true
}
