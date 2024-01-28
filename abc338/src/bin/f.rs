use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uvw: [(Usize1, Usize1, i32); m],
    }

    let calc_dists = |start: usize| {
        let mut dists: Vec<Option<i32>> = vec![None; n];
        dists[start] = Some(0);
        for _ in 0..n {
            let mut updated = false;
            for &(u, v, w) in &uvw {
                if let Some(src_dist) = dists[u] {
                    updated |= chmin_for_option(&mut dists[v], src_dist + w);
                }
            }

            if !updated {
                return dists;
            }
        }

        panic!("Negative cycles exist.");
    };

    let dist_mat = (0..n).map(|start| calc_dists(start)).collect_vec();

    let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = Some(0);
    }
    for bit in 1..1 << n {
        for from in 0..n {
            if bit >> from & 1 == 0 {
                continue;
            }

            for to in 0..n {
                if bit >> to & 1 == 1 {
                    continue;
                }

                if let (Some(src_dist), Some(add_dist)) = (dp[bit][from], dist_mat[from][to]) {
                    chmin_for_option(&mut dp[bit | 1 << to][to], src_dist + add_dist);
                }
            }
        }
    }

    let ans = dp[(1 << n) - 1].iter().filter_map(|&dist| dist).min();
    match ans {
        Some(ans) => println!("{}", ans),
        None => println!("No"),
    }
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
