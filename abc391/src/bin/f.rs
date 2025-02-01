use std::{cmp::Reverse, collections::BinaryHeap};

use im_rc::HashSet;
use itertools::{izip, Itertools};
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut sequences: [[usize; n]; 3],
    }

    let mut visited = HashSet::<[usize; 3]>::new();

    sequences
        .iter_mut()
        .for_each(|seq| seq.sort_unstable_by_key(|&v| Reverse(v)));

    let calc_sum = |progresses: [usize; 3]| {
        let (a, b, c) = izip!(&sequences, progresses)
            .map(|(seq, progress)| seq[progress])
            .collect_tuple()
            .unwrap();
        a * b + b * c + c * a
    };

    let init_node = Node::new([0; 3], calc_sum([0; 3]));
    let mut heap = BinaryHeap::from([init_node]);

    let mut update_heap = |heap: &mut BinaryHeap<Node>| {
        let Node { progresses, value } = heap.pop().unwrap();
        if visited.contains(&progresses) {
            return None;
        }

        visited.insert(progresses);

        for i in 0..3 {
            if progresses[i] + 1 < n {
                let mut next_progresses = progresses;
                next_progresses[i] += 1;

                heap.push(Node {
                    progresses: next_progresses,
                    value: calc_sum(next_progresses),
                });
            }
        }

        Some(value)
    };

    let ans = (0_usize..)
        .filter_map(|_| update_heap(&mut heap))
        .nth(k - 1)
        .unwrap();
    println!("{}", ans);
}

#[derive(Debug, Clone, Copy)]
struct Node {
    progresses: [usize; 3],
    value: usize,
}

impl Node {
    fn new(progresses: [usize; 3], value: usize) -> Self {
        Self { progresses, value }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
