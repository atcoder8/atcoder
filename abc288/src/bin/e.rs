use itertools::{enumerate, izip, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
        cc: [usize; n],
        xx: [Usize1; m],
    }

    let mut desired_flags = vec![false; n];
    for &x in &xx {
        desired_flags[x] = true;
    }

    let calc_suffix_sum_cost = |item_idx: usize| {
        let mut suffix_sum_cost = vec![cc[item_idx]; item_idx + 1];
        for bought_num in 1..=item_idx {
            suffix_sum_cost[bought_num] =
                suffix_sum_cost[bought_num - 1].min(cc[item_idx - bought_num]);
        }

        suffix_sum_cost
    };

    let cost_mat = (0..n)
        .map(|item_idx| calc_suffix_sum_cost(item_idx))
        .collect_vec();

    let mut dp = vec![Some(0)];
    for (item_idx, (&a, &desired)) in enumerate(izip!(&aa, &desired_flags)) {
        let mut next_dp = vec![None; item_idx + 2];
        if !desired {
            next_dp[..=item_idx].copy_from_slice(&dp);
        }

        for prev_bought_num in 0..=item_idx {
            let Some(prev_cost) = dp[prev_bought_num] else { continue; };

            let cand_cost = prev_cost + a + cost_mat[item_idx][prev_bought_num];
            update_cost(&mut next_dp[prev_bought_num + 1], cand_cost);
        }

        dp = next_dp;
    }

    let ans = dp.iter().filter_map(|&x| x).min().unwrap();
    println!("{}", ans);
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
