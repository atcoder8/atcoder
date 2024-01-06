use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut queue = VecDeque::from((1..=n).map(|x| (x as i64, 0_i64)).collect_vec());
    for _ in 0..q {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                c: char,
            }

            let (head_x, head_y) = *queue.front().unwrap();

            let next_head = match c {
                'R' => (head_x + 1, head_y),
                'L' => (head_x - 1, head_y),
                'U' => (head_x, head_y + 1),
                'D' => (head_x, head_y - 1),
                _ => unreachable!(),
            };

            queue.pop_back();
            queue.push_front(next_head);
        } else {
            input! {
                q: Usize1,
            }

            let (x, y) = queue[q];
            println!("{x} {y}");
        }
    }
}
