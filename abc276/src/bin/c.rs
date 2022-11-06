use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n],
    }

    let mut inc_cnt = 1;

    for i in (0..(n - 1)).rev() {
        if pp[i] < pp[i + 1] {
            inc_cnt += 1;
        } else {
            break;
        }
    }

    let bound = pp[n - 1 - inc_cnt];
    let mut inc_seq = pp.iter().cloned().rev().take(inc_cnt).collect_vec();
    let head = inc_seq
        .iter()
        .enumerate()
        .filter(|(_, &x)| x < bound)
        .max_by_key(|(_, &x)| x)
        .unwrap()
        .0;
    let head = inc_seq.remove(head);
    inc_seq.push(bound);
    inc_seq.sort_by_key(|&x| Reverse(x));

    for &p in pp.iter().take(n - inc_cnt - 1) {
        print!("{} ", p + 1);
    }
    print!("{}", head + 1);
    for &x in &inc_seq {
        print!(" {}", x + 1);
    }
    println!();
}
