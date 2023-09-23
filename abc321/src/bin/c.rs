use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        k: usize,
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    for i in 1..10 {
        queue.push_back(i);
    }

    let mut cnt = 0;
    let ans = loop {
        let cur = queue.pop_front().unwrap();
        cnt += 1;

        if cnt == k {
            break cur;
        }

        for i in 0..(cur % 10) {
            let next = 10 * cur + i;
            queue.push_back(next);
        }
    };

    println!("{}", ans);
}
