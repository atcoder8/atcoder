use std::{cmp::Ordering, io::Write};

use itertools::Itertools;
use proconio::input_interactive;

fn main() {
    input_interactive!(n: usize);

    let mut orders = (0..n).collect_vec();
    orders.sort_by(|&x, &y| ask_compare(x, y));

    while orders.len() >= 2 {
        let i = orders.remove(0);
        let j = orders.pop().unwrap();

        let p = request_add(i, j);

        let ins_pos = orders.partition_point(|&x| ask_compare(p, x).is_gt());
        orders.insert(ins_pos, p);
    }

    finish();
}

fn ask_compare(i: usize, j: usize) -> Ordering {
    if i == j {
        return Ordering::Equal;
    }

    println!("? {} {}", i + 1, j + 1);
    std::io::stdout().flush().unwrap();
    input_interactive!(lt: usize);

    if lt == 1 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn request_add(i: usize, j: usize) -> usize {
    println!("+ {} {}", i + 1, j + 1);
    std::io::stdout().flush().unwrap();
    input_interactive!(p: usize);

    p - 1
}

fn finish() {
    println!("!");
    std::io::stdout().flush().unwrap();
}
