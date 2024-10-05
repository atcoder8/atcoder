use std::{collections::BTreeMap, ops::Range};

use ndarray::prelude::*;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, k, x): (usize, usize, usize),
        tt: [usize; n],
    }

    let get_num_orders_range =
        |range: Range<usize>| tt.lower_bound(&range.end) - tt.lower_bound(&range.start);

    let mut min_sum_cost: Option<usize> = None;
    let mut init_costs = Array2::<Option<usize>>::from_elem((n + 1, n + 1), None);
    init_costs[(0, 0)] = Some(0);
    let mut dp = BTreeMap::from([(0_usize, init_costs)]);
    while let Some((day, costs)) = dp.pop_first() {
        for ((num_orders, num_shipments), &sum_cost) in costs.indexed_iter() {
            let Some(sum_cost) = sum_cost else {
                continue;
            };

            assert!(num_orders >= num_shipments);

            // 不満度の総和の最小値を更新
            if num_shipments == n {
                chmin_for_option(&mut min_sum_cost, sum_cost);
                continue;
            }

            // 未処理のオーダーの数
            let rem_orders = num_orders - num_shipments;

            // 出荷せずに次の注文を待つ場合の遷移
            let next_order_day = tt.get(tt.upper_bound(&day)).cloned();
            if let Some(next_order_day) = next_order_day {
                let next_costs = dp
                    .entry(next_order_day)
                    .or_insert(Array2::from_elem((n + 1, n + 1), None));
                let add_orders = get_num_orders_range(next_order_day..next_order_day + 1);
                chmin_for_option(
                    &mut next_costs[(num_orders + add_orders, num_shipments)],
                    sum_cost + rem_orders * (next_order_day - day),
                );
            }

            // 出荷する場合の遷移
            if rem_orders != 0 {
                let add_shipments = k.min(rem_orders);
                let reduced_rem_orders = rem_orders - add_shipments;
                let next_day = day + x;
                let next_costs = dp
                    .entry(next_day)
                    .or_insert(Array2::from_elem((n + 1, n + 1), None));
                let left = tt.lower_bound(&(day + 1));
                let right = tt.upper_bound(&next_day);
                let add_cost = reduced_rem_orders * (next_day - day)
                    + tt[left..right].iter().map(|&t| next_day - t).sum::<usize>();
                chmin_for_option(
                    &mut next_costs[(num_orders + right - left, num_shipments + add_shipments)],
                    sum_cost + add_cost,
                );
            }
        }
    }

    println!("{}", min_sum_cost.unwrap());
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
