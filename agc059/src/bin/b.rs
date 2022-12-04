// Not solved yet.

use std::collections::{HashSet, VecDeque};

use itertools::{join, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        cc: [Usize1; n],
    }

    if cc.iter().all_equal() {
        println!("{}", join(&cc, " "));

        return;
    }

    let mut counts = vec![0_usize; n];
    for &c in &cc {
        counts[c] += 1;
    }
    let mut count_per_color = counts
        .iter()
        .enumerate()
        .filter(|&(_, &count)| count != 0)
        .map(|(i, &count)| (i, count))
        .collect_vec();
    count_per_color.sort_by_key(|&(_, count)| count);

    let mut from_deque = VecDeque::from(count_per_color.clone());
    let mut ans = Vec::new();

    while !from_deque.is_empty() {
        let (wrap_color, mut wrap_rem) = from_deque.pop_back().unwrap();

        ans.push(wrap_color);
        wrap_rem -= 1;

        while !from_deque.is_empty() && wrap_rem != 0 {
            let (clump_color, clump_count) = from_deque.pop_front().unwrap();
            ans.extend(vec![clump_color; clump_count]);
            ans.push(wrap_color);
            wrap_rem -= 1;
        }

        ans.append(&mut vec![wrap_color; wrap_rem]);
    }

    if count_pair_num(&ans) > count_per_color.len() - 1 {
        ans = vec![];
        for &(color, count) in &count_per_color {
            ans.append(&mut vec![color; count]);
        }
    }

    ans.iter_mut().for_each(|x| *x += 1);
    println!("{}", join(&ans, " "));
}

pub fn count_pair_num(ball_colors: &Vec<usize>) -> usize {
    let mut pairs = HashSet::new();

    for i in 0..ball_colors.len() {
        let c1 = ball_colors[i];
        let c2 = ball_colors[(i + 1) % ball_colors.len()];

        if c1 < c2 {
            pairs.insert((c1, c2));
        } else if c1 > c2 {
            pairs.insert((c2, c1));
        }
    }

    pairs.len()
}
