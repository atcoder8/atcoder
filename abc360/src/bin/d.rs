use std::collections::VecDeque;

use itertools::izip;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, t): (usize, i64),
        s: Chars,
        xx: [i64; n],
    }

    let mut to_left = vec![];
    let mut to_right = vec![];
    for (&c, &x) in izip!(&s, &xx) {
        if c == '0' {
            to_left.push(x);
        } else {
            to_right.push(x);
        }
    }
    to_left.sort_unstable();
    to_right.sort_unstable();

    let mut ans = 0;
    let mut queue = VecDeque::new();
    let mut pos = 0;
    for right_end in to_left {
        while pos < to_right.len() && to_right[pos] <= right_end {
            queue.push_back(to_right[pos]);
            pos += 1;
        }

        while queue
            .front()
            .is_some_and(|left_end| left_end + t < right_end - t)
        {
            queue.pop_front();
        }

        ans += queue.len();
    }

    println!("{}", ans);
}
