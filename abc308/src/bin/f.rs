use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut pp: [usize; n],
        ll: [usize; m],
        dd: [usize; m],
    }

    pp.sort_unstable();

    let mut ans = 0;
    let coupons = (0..m).sorted_by_key(|&i| ll[i]).collect_vec();
    let mut heap = BinaryHeap::new();
    let mut cnt = 0;
    for &p in &pp {
        while cnt < m && ll[coupons[cnt]] <= p {
            heap.push(dd[coupons[cnt]]);
            cnt += 1;
        }

        ans += if let Some(d) = heap.pop() { p - d } else { p };
    }

    println!("{}", ans);
}
