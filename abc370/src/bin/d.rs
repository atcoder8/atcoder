use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (h, w, q): (usize, usize, usize),
        rc: [(Usize1, Usize1); q],
    }

    let mut horizontal_walls: Vec<BTreeSet<usize>> = vec![(0..w).collect(); h];
    let mut vertical_walls: Vec<BTreeSet<usize>> = vec![(0..h).collect(); w];
    for &(r, c) in &rc {
        if horizontal_walls[r].contains(&c) {
            horizontal_walls[r].remove(&c);
            vertical_walls[c].remove(&r);
            continue;
        }

        let mut delete_coords = vec![];
        if let Some(&other_c) = horizontal_walls[r].range(..c).next_back() {
            delete_coords.push((r, other_c));
        }
        if let Some(&other_c) = horizontal_walls[r].range(c + 1..).next() {
            delete_coords.push((r, other_c));
        }
        if let Some(&other_r) = vertical_walls[c].range(..r).next_back() {
            delete_coords.push((other_r, c));
        }
        if let Some(&other_r) = vertical_walls[c].range(r + 1..).next() {
            delete_coords.push((other_r, c));
        }

        for &(delete_r, delete_c) in &delete_coords {
            horizontal_walls[delete_r].remove(&delete_c);
            vertical_walls[delete_c].remove(&delete_r);
        }
    }

    let ans = horizontal_walls
        .iter()
        .map(|walls| walls.len())
        .sum::<usize>();
    println!("{}", ans);
}
