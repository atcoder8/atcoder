use itertools::{iproduct, Itertools};
use ndarray::prelude::*;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uvt: [(Usize1, Usize1, usize); m],
        q: usize,
        bbb: [[Usize1]; q],
    }

    let mut dist_mat = Array2::from_elem((n, n), None::<usize>);
    for i in 0..n {
        dist_mat[(i, i)] = Some(0);
    }
    for &(u, v, t) in &uvt {
        chmin_for_option(&mut dist_mat[(u, v)], t);
        chmin_for_option(&mut dist_mat[(v, u)], t);
    }
    for (mid, from, to) in iproduct!(0..n, 0..n, 0..n) {
        if let (Some(dist1), Some(dist2)) = (dist_mat[(from, mid)], dist_mat[(mid, to)]) {
            chmin_for_option(&mut dist_mat[(from, to)], dist1 + dist2);
        }
    }

    let dist_mat = dist_mat.map(|dist| dist.unwrap());

    let solve = |bridges: &[usize]| {
        let find_min_cost = |perm: &[usize]| {
            let first_bridge = perm[0];
            let (first_u, first_v, first_t) = uvt[first_bridge];

            let mut dp = [
                dist_mat[(0, first_v)] + first_t,
                dist_mat[(0, first_u)] + first_t,
            ];

            for (&prev_bridge, &bridge) in perm.iter().tuple_windows() {
                let (prev_u, prev_v, _) = uvt[prev_bridge];
                let (u, v, t) = uvt[bridge];
                dp = [
                    (dp[0] + dist_mat[(prev_u, v)] + t).min(dp[1] + dist_mat[(prev_v, v)] + t),
                    (dp[0] + dist_mat[(prev_u, u)] + t).min(dp[1] + dist_mat[(prev_v, u)] + t),
                ];
            }

            let last_bridge = *perm.last().unwrap();
            let (last_u, last_v, _) = uvt[last_bridge];
            (dp[0] + dist_mat[(last_u, n - 1)]).min(dp[1] + dist_mat[(last_v, n - 1)])
        };

        bridges
            .iter()
            .cloned()
            .permutations(bridges.len())
            .map(|perm| find_min_cost(&perm))
            .min()
            .unwrap()
    };

    let ans = bbb.iter().map(|bb| solve(bb)).join("\n");
    println!("{}", ans);
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
