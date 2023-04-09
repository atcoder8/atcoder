use std::{cmp::Reverse, collections::BinaryHeap};

use im_rc::HashSet;
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    heap.push(Reverse(0));
    let mut used: HashSet<usize> = HashSet::new();
    let mut cnt = 0;

    loop {
        let Reverse(cur) = heap.pop().unwrap();

        if used.contains(&cur) {
            continue;
        }

        used.insert(cur);

        cnt += 1;

        if cnt == k + 1 {
            return cur;
        }

        for &a in &aa {
            let next = cur + a;

            if !used.contains(&next) {
                heap.push(Reverse(next));
            }
        }
    }
}
