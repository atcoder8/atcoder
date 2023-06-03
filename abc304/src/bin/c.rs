use std::collections::VecDeque;

use proconio::input;

type Coord = (i64, i64);

fn main() {
    input! {
        (n, d): (usize, i64),
        xy: [(i64, i64); n],
    }

    let mut visited = vec![false; n];
    visited[0] = true;
    let mut queue = VecDeque::from(vec![0]);
    while let Some(cur) = queue.pop_front() {
        for next in 0..n {
            if calc_sq_dist(xy[cur], xy[next]) <= d * d {
                if visited[next] {
                    continue;
                }

                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    for i in 0..n {
        println!("{}", if visited[i] { "Yes" } else { "No" });
    }
}

fn calc_sq_dist(coord1: Coord, coord2: Coord) -> i64 {
    let (x1, y1) = coord1;
    let (x2, y2) = coord2;

    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}
