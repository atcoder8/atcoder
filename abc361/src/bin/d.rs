use std::collections::VecDeque;

use im_rc::HashSet;
use itertools::enumerate;
use proconio::{input, marker::Chars};

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Disc {
    Empty,
    Black,
    White,
}

fn chars_to_discs(cc: &[char]) -> Vec<Disc> {
    let mut discs = vec![Disc::Empty; cc.len() + 2];
    for (i, &c) in enumerate(cc) {
        discs[i] = if c == 'W' { Disc::White } else { Disc::Black };
    }

    discs
}

fn solve() -> Option<usize> {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    let start_discs = chars_to_discs(&s);
    let goal_discs = chars_to_discs(&t);
    let mut pool = HashSet::<Vec<Disc>>::new();

    let mut queue = VecDeque::from([(start_discs, n, 0)]);
    while let Some((discs, empty_pos, cnt)) = queue.pop_front() {
        if discs == goal_discs {
            return Some(cnt);
        }

        if pool.contains(&discs) {
            continue;
        }

        pool.insert(discs.clone());

        for x in 0..n + 1 {
            if x + 1 < empty_pos || empty_pos + 1 < x {
                let mut next_discs = discs.clone();
                next_discs.swap(x, empty_pos);
                next_discs.swap(x + 1, empty_pos + 1);
                queue.push_back((next_discs, x, cnt + 1));
            }
        }
    }

    None
}
