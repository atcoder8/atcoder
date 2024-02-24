use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ldkcab: [(usize, usize, usize, usize, Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(l, d, k, c, a, b) in &ldkcab {
        graph[b].push((a, l, d, k, c));
    }

    let mut latest_times = vec![None; n];
    let mut heap = BinaryHeap::from([(2 * 10_usize.pow(18), n - 1)]);
    while let Some((last_time, cur)) = heap.pop() {
        if latest_times[cur].is_some() {
            continue;
        }

        latest_times[cur] = Some(last_time);

        for &(next, l, d, k, c) in &graph[cur] {
            if l + c > last_time {
                continue;
            }

            let mut ok = 0_usize;
            let mut ng = k;
            while ok.abs_diff(ng) > 1 {
                let mid = (ok + ng) / 2;

                if l + mid * d + c <= last_time {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }

            heap.push((l + ok * d, next));
        }
    }

    let ans = latest_times[..n - 1]
        .iter()
        .map(|latest_time| match latest_time {
            Some(latest_time) => latest_time.to_string(),
            None => "Unreachable".to_owned(),
        })
        .join("\n");
    println!("{}", ans);
}
