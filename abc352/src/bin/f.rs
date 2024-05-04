// unfinished

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }

    let orders = match rec(&abc, vec![None; n], 0) {
        Some(orders) => orders,
        None => vec![None; n],
    };

    let ans = orders
        .iter()
        .map(|order| match order {
            Some(order) => format!("{}", order + 1),
            None => "-1".to_owned(),
        })
        .join(" ");
    println!("{}", ans);
}

fn rec(
    abc: &[(usize, usize, usize)],
    orders: Vec<Option<usize>>,
    pos: usize,
) -> Option<Vec<Option<usize>>> {
    if pos == abc.len() {
        return Some(orders);
    }

    let n = orders.len();

    let (a, b, c) = abc[pos];
    let mut candidates = vec![];
    for x in c..n {
        let y = x - c;

        if orders[x].is_some() || orders[y].is_some() {
            continue;
        }

        let mut next_orders = orders.clone();
        next_orders[x] = Some(a);
        next_orders[y] = Some(b);

        if let Some(orders) = rec(&abc, next_orders, pos + 1) {
            candidates.push(orders);
        }
    }

    if candidates.len() == 1 {
        Some(candidates[0].clone())
    } else {
        None
    }
}
