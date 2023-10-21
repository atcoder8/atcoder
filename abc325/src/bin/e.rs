use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        (n, a, b, c): (usize, usize, usize, usize),
        ddd: [[usize; n]; n],
    }

    let mut car_costs = vec![None; n];
    let mut car_heap = BinaryHeap::from(vec![(Reverse(0), 0)]);
    while let Some((Reverse(cost), cur)) = car_heap.pop() {
        if car_costs[cur].is_some_and(|actual_cost| cost != actual_cost) {
            continue;
        }

        for next in 0..n {
            let next_cost = &mut car_costs[next];
            let candidate_cost = cost + ddd[cur][next] * a;

            if next_cost.is_none() || candidate_cost < next_cost.unwrap() {
                *next_cost = Some(candidate_cost);
                car_heap.push((Reverse(candidate_cost), next));
            }
        }
    }

    let mut train_costs = vec![None; n];
    let mut train_heap = BinaryHeap::from(vec![(Reverse(0), n - 1)]);
    while let Some((Reverse(cost), cur)) = train_heap.pop() {
        if train_costs[cur].is_some_and(|actual_cost| cost != actual_cost) {
            continue;
        }

        for next in 0..n {
            let next_cost = &mut train_costs[next];
            let candidate_cost = cost + ddd[cur][next] * b + c;

            if next_cost.is_none() || candidate_cost < next_cost.unwrap() {
                *next_cost = Some(candidate_cost);
                train_heap.push((Reverse(candidate_cost), next));
            }
        }
    }

    let ans = car_costs
        .iter()
        .zip(&train_costs)
        .map(|(&car_cost, &train_cost)| car_cost.unwrap() + train_cost.unwrap())
        .chain([car_costs[n - 1].unwrap()])
        .min()
        .unwrap();
    println!("{}", ans);
}
