use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = aa.iter().sum::<usize>();
    let mut nodes = BinaryHeap::from(
        aa.iter()
            .map(|&a| Reverse(Node { a, degree: 1 }))
            .collect_vec(),
    );
    for _ in 0..n - 2 {
        let mut node = nodes.pop().unwrap().0;

        ans += node.add_cost();

        node.degree += 1;
        nodes.push(Reverse(node));
    }

    println!("{}", ans);
}

struct Node {
    a: usize,
    degree: usize,
}

impl Node {
    fn add_cost(&self) -> usize {
        (2 * self.degree + 1) * self.a
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.add_cost() == other.add_cost()
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.add_cost().partial_cmp(&other.add_cost())
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
