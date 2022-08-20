// unfinished

use std::collections::{BinaryHeap, HashSet};

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let xx: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect()
    };
    let cc: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    let mut rev_xx = vec![vec![]; n];
    for (i, x) in xx.iter().enumerate() {
        rev_xx[*x].push(i);
    }

    let mut total_costs = vec![0_usize; n];
    for p in 0..n {
        total_costs[p] += rev_xx[p].iter().map(|x| cc[*x]).sum::<usize>();
    }

    println!("{:?}", total_costs);

    let mut heap = BinaryHeap::new();
    for (i, total_cost) in total_costs.iter().enumerate() {
        heap.push((*total_cost, i));
    }

    let mut ans = 0;
    let mut pp = vec![];
    let mut used = HashSet::new();
    while let Some((cost, p)) = heap.pop() {
        if used.contains(&p) {
            continue;
        }

        pp.push(p);
        used.insert(p);
        ans += cost;
        rev_xx[p].iter().for_each(|x| {
            total_costs[*x] -= cc[*x];
            heap.push((total_costs[*x], *x));
        });
    }

    println!("{}", ans);
}
