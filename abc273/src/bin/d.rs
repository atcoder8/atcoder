use std::collections::HashMap;

use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        (h, w, rs, cs): (usize, usize, Usize1, Usize1),
        n: usize,
        rc: [(Usize1, Usize1); n],
        q: usize,
        dl: [(char, usize); q],
    }

    let mut horizontal_walls: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut vertical_walls: HashMap<usize, Vec<usize>> = HashMap::new();

    for &(r, c) in &rc {
        horizontal_walls.entry(r).or_insert(vec![]).push(c);
        vertical_walls.entry(c).or_insert(vec![]).push(r);
    }

    horizontal_walls
        .values_mut()
        .for_each(|x| x.sort_unstable());
    vertical_walls.values_mut().for_each(|x| x.sort_unstable());

    let (mut cur_r, mut cur_c) = (rs, cs);

    for &(d, l) in &dl {
        match d {
            'U' => {
                cur_r = vertical_walls
                    .get(&cur_c)
                    .and_then(|walls| {
                        walls
                            .upper_bound(&cur_r)
                            .checked_sub(1)
                            .map(|upper_wall_idx| walls[upper_wall_idx] + 1)
                    })
                    .unwrap_or(0)
                    .max(cur_r.saturating_sub(l));
            }
            'D' => {
                cur_r = vertical_walls
                    .get(&cur_c)
                    .and_then(|walls| {
                        walls
                            .get(walls.upper_bound(&cur_r))
                            .map(|&lower_wall_r| lower_wall_r - 1)
                    })
                    .unwrap_or(h - 1)
                    .min(cur_r + l);
            }
            'L' => {
                cur_c = horizontal_walls
                    .get(&cur_r)
                    .and_then(|walls| {
                        walls
                            .upper_bound(&cur_c)
                            .checked_sub(1)
                            .map(|left_wall_idx| walls[left_wall_idx] + 1)
                    })
                    .unwrap_or(0)
                    .max(cur_c.saturating_sub(l));
            }
            'R' => {
                cur_c = horizontal_walls
                    .get(&cur_r)
                    .and_then(|walls| {
                        walls
                            .get(walls.upper_bound(&cur_c))
                            .map(|&right_wall_c| right_wall_c - 1)
                    })
                    .unwrap_or(w - 1)
                    .min(cur_c + l);
            }
            _ => panic!(),
        }

        println!("{} {}", cur_r + 1, cur_c + 1);
    }
}
