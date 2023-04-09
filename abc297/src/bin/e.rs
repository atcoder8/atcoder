use std::{cmp::Reverse, collections::BinaryHeap};

use im_rc::HashSet;
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, k): (usize, usize),
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    heap.push(Reverse(0));

    let mut used: HashSet<usize> = HashSet::new();
    used.insert(0);

    let mut cnt = 0;

    while let Some(Reverse(cur)) = heap.pop() {
        used.insert(cur);

        cnt += 1;

        if cnt == k + 1 {
            return cur;
        }

        for &a in &aa {
            let next = cur + a;

            if !used.contains(&next) {
                used.insert(next);
                heap.push(Reverse(next));
            }
        }
    }

    unreachable!()
}
