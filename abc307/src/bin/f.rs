use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uvw: [(Usize1, Usize1, usize); m],
        k: usize,
        aa: [Usize1; k],
        d: usize,
        xx: [usize; d],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    graph
        .iter_mut()
        .for_each(|edges| edges.sort_unstable_by_key(|edge| edge.1));

    let mut infected_days = vec![None; n];
    let mut heap = BinaryHeap::new();
    for &a in &aa {
        infected_days[a] = Some(0);

        for &(next, weight) in &graph[a] {
            heap.push((Reverse(weight), next));
        }
    }

    for (day, x) in enumerate(xx) {
        let mut next_heap = BinaryHeap::new();

        while let Some(&(Reverse(cost), cur)) = heap.peek() {
            if cost > x {
                break;
            }

            heap.pop();

            if infected_days[cur].is_some() {
                continue;
            }

            infected_days[cur] = Some(day + 1);

            for &(next, weight) in &graph[cur] {
                heap.push((Reverse(cost + weight), next));
                next_heap.push((Reverse(weight), next));
            }
        }

        heap.append(&mut next_heap);
    }

    for infected_day in infected_days {
        match infected_day {
            Some(ans) => println!("{}", ans),
            None => println!("-1"),
        }
    }
}
