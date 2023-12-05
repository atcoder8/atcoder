use proconio::input;

fn main() {
    input! {
        (n, l): (usize, usize),
        aa: [usize; n],
    }

    let calc_cost = |p1: usize, p2: usize| 2 * (l + (aa[p1] + aa[p2]).abs_diff(l));

    let mut ans = None;
    let mut right = n - 1;
    for left in 0..n {
        if left > right {
            break;
        }

        while right > left && aa[left] + aa[right] > l {
            right -= 1;
        }

        update_cost(&mut ans, calc_cost(left, right));
        if right + 1 < n {
            update_cost(&mut ans, calc_cost(left, right + 1));
        }
    }

    println!("{}", ans.unwrap());
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
    T: Ord,
{
    if cost.as_ref().is_some_and(|score| score <= &cand_cost) {
        return false;
    }

    *cost = Some(cand_cost);

    true
}
