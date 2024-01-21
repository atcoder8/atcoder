use itertools::{enumerate, Itertools};
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut val_to_pos = vec![vec![]; 11];
    for (pos, &a) in enumerate(&aa) {
        val_to_pos[a].push(pos);
    }

    let find_min_right_idx = |left_idx: usize| {
        let left = aa[left_idx];

        let mut min_right = None;
        for mid in 0..=10 {
            if 2 * mid < left || 2 * mid - left > 10 {
                continue;
            }

            let mid_pos = val_to_pos[mid].upper_bound(&left_idx);
            let Some(&mid_idx) = val_to_pos[mid].get(mid_pos) else { continue; };

            let right = 2 * mid - left;
            let right_pos = val_to_pos[right].upper_bound(&mid_idx);
            let Some(&right_idx) = val_to_pos[right].get(right_pos) else { continue; };

            update_cost(&mut min_right, right_idx);
        }

        min_right
    };

    let mut min_right_indexes = (0..n).map(find_min_right_idx).collect_vec();
    for i in (0..n - 1).rev() {
        if let Some(x) = min_right_indexes[i + 1] {
            update_cost(&mut min_right_indexes[i], x);
        }
    }

    let mut ans = 0;
    for left_idx in 0..n {
        let Some(min_right_idx) = min_right_indexes[left_idx] else { continue; };
        ans += n - min_right_idx;
    }

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
