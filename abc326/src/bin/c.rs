use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let mut ans = 0;
    let mut queue = VecDeque::new();
    for &a in &aa {
        queue.push_back(a);
        while a - queue.front().unwrap() >= m {
            queue.pop_front();
        }

        ans = ans.max(queue.len());
    }

    println!("{}", ans);
}
