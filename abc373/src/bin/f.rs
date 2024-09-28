use std::collections::BinaryHeap;

use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, w): (usize, usize),
        wv: [(usize, usize); n],
    }

    let mut heap_each_weight: Vec<BinaryHeap<(usize, usize)>> = vec![BinaryHeap::new(); w + 1];
    for &(w, v) in &wv {
        heap_each_weight[w].push((v - 1, 1));
    }

    let mut dp = vec![0_usize; w + 1];
    let mut next_dp = vec![0_usize; w + 1];
    for (weight, heap) in enumerate(&mut heap_each_weight) {
        if heap.is_empty() {
            continue;
        }

        next_dp.copy_from_slice(&dp);

        let mut add_value = 0_usize;
        for num_items in 1..=w / weight {
            let Some((value, k)) = heap.pop() else {
                break;
            };

            add_value += value;

            if value > 2 {
                heap.push((value - 2, k + 1));
            }

            for (sum_weight, &sum_value) in enumerate(&dp) {
                let added_weight = sum_weight + weight * num_items;
                if added_weight > w {
                    break;
                }
                next_dp[added_weight] = next_dp[added_weight].max(sum_value + add_value);
            }
        }

        std::mem::swap(&mut dp, &mut next_dp);
    }

    let ans = dp.iter().max().unwrap();
    println!("{}", ans);
}
