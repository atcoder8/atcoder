use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let ans = find_max_length(&aa).max(find_max_length(&aa[1..]));
    println!("{}", ans);
}

fn find_max_length(aa: &[usize]) -> usize {
    let mut max_length = 0_usize;
    let mut right = 0_usize;
    let mut counts = vec![0_usize; aa.len() + 1];
    let mut queue = VecDeque::<usize>::new();
    while 2 * right + 1 < aa.len() {
        if aa[2 * right] != aa[2 * right + 1] {
            right += 1;
            while let Some(a) = queue.pop_front() {
                counts[a] -= 1;
            }

            continue;
        }

        let pushed_a = aa[2 * right];
        counts[pushed_a] += 1;
        queue.push_back(pushed_a);
        while counts[pushed_a] >= 2 {
            let a = queue.pop_front().unwrap();
            counts[a] -= 1;
        }

        max_length = max_length.max(2 * queue.len());
        right += 1;
    }

    max_length
}
