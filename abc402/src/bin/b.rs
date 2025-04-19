use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut ans = vec![];
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                x: usize,
            }

            queue.push_back(x);
        } else {
            ans.push(queue.pop_front().unwrap());
        }
    }

    println!("{}", ans.iter().join("\n"));
}
