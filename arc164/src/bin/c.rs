use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }

    let mut heap = BinaryHeap::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        heap.push((a - b, i, false));
    }

    let mut ans = 0;
    for _ in 0..n {
        let (_, idx, rev) = heap.pop().unwrap();
        let (a, b) = ab[idx];
        if rev {
            heap.push((a - b, idx, false));
        } else {
            heap.push((b - a, idx, true));
        }

        let (_, idx, rev) = heap.pop().unwrap();
        let (a, b) = ab[idx];
        if rev {
            ans += b;
        } else {
            ans += a;
        }
    }

    println!("{}", ans);
}
