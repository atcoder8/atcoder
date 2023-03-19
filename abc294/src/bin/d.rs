use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut heap = BinaryHeap::new();
    let mut used = vec![false; n];
    let mut call_cnt = 0;

    for _ in 0..q {
        input! {
            event_type: usize,
        }

        if event_type == 1 {
            heap.push(Reverse(call_cnt));
            call_cnt += 1;
        } else if event_type == 2 {
            input! {
                x: Usize1,
            }

            used[x] = true;
        } else {
            let ans = loop {
                let Reverse(ans) = heap.pop().unwrap();
                if !used[ans] {
                    break ans;
                }
            };
            heap.push(Reverse(ans));
            println!("{}", ans + 1);
        }
    }
}
