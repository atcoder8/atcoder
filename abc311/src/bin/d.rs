use std::collections::VecDeque;

use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        (n, m): (usize, usize),
        ss: [Chars; n],
    }

    let mut horizontal = vec![vec![]; n];
    let mut vertical = vec![vec![]; m];
    for i in 0..n {
        for j in 0..m {
            if ss[i][j] == '#' {
                horizontal[i].push(j);
                vertical[j].push(i);
            }
        }
    }
    horizontal.iter_mut().for_each(|x| x.sort_unstable());
    vertical.iter_mut().for_each(|x| x.sort_unstable());

    let upper = |i: usize, j: usize| (vertical[j][vertical[j].upper_bound(&i) - 1] + 1, j);
    let lower = |i: usize, j: usize| (vertical[j][vertical[j].upper_bound(&i)] - 1, j);
    let left = |i: usize, j: usize| (i, horizontal[i][horizontal[i].upper_bound(&j) - 1] + 1);
    let right = |i: usize, j: usize| (i, horizontal[i][horizontal[i].upper_bound(&j)] - 1);

    let mut touch = vec![vec![0; m]; n];

    let mut visited = vec![vec![false; m]; n];
    let mut queue = VecDeque::from(vec![(1, 1)]);
    while let Some((x, y)) = queue.pop_front() {
        let next_coords = [upper(x, y), lower(x, y), left(x, y), right(x, y)];
        for (dir_idx, &(next_x, next_y)) in next_coords.iter().enumerate() {
            if (touch[next_x][next_y] >> dir_idx) & 1 == 0 {
                touch[next_x][next_y] |= 1 << dir_idx;
                for i in x.min(next_x)..=x.max(next_x) {
                    for j in y.min(next_y)..=y.max(next_y) {
                        visited[i][j] = true;
                    }
                }
                queue.push_back((next_x, next_y));
            }
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            ans += visited[i][j] as usize;
        }
    }
    println!("{}", ans);
}
