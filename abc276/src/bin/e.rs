use std::collections::VecDeque;

use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, _w): (usize, usize),
        ccc: [Chars; h],
    }

    println!("{}", if solve_by_bfs(&ccc) { "Yes" } else { "No" });
}

fn find_start_coord(ccc: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let (h, w) = (ccc.len(), ccc[0].len());

    for i in 0..h {
        for j in 0..w {
            if ccc[i][j] == 'S' {
                return Some((i, j));
            }
        }
    }

    None
}

#[allow(unused)]
fn solve_by_union_find(ccc: &Vec<Vec<char>>) -> bool {
    let (h, w) = (ccc.len(), ccc[0].len());

    let coord_to_idx = |(row, col)| row * w + col;

    let mut uf: UnionFind<usize> = UnionFind::new(h * w);

    for row in 0..h {
        for col in 0..w {
            if ccc[row][col] != '.' {
                continue;
            }

            if row < h - 1 && ccc[row + 1][col] == '.' {
                uf.union(coord_to_idx((row, col)), coord_to_idx((row + 1, col)));
            }

            if col < w - 1 && ccc[row][col + 1] == '.' {
                uf.union(coord_to_idx((row, col)), coord_to_idx((row, col + 1)));
            }
        }
    }

    let (start_row, start_col) = find_start_coord(ccc).unwrap();

    let coords_to_check = DIFFS
        .iter()
        .filter_map(|&(diff_row, diff_col)| {
            let (next_row, next_col) = (
                start_row.wrapping_add(diff_row),
                start_col.wrapping_add(diff_col),
            );

            if next_row < h && next_col < w {
                Some((next_row, next_col))
            } else {
                None
            }
        })
        .collect_vec();

    for i in 0..(coords_to_check.len() + 1) {
        for j in (i + 1)..coords_to_check.len() {
            if uf.equiv(
                coord_to_idx(coords_to_check[i]),
                coord_to_idx(coords_to_check[j]),
            ) {
                return true;
            }
        }
    }

    false
}

fn solve_by_bfs(ccc: &Vec<Vec<char>>) -> bool {
    let (h, w) = (ccc.len(), ccc[0].len());

    let coord_to_idx = |(row, col)| row * w + col;

    let mut que = VecDeque::new();
    let mut visited = vec![0; h * w];

    let (start_row, start_col) = find_start_coord(ccc).unwrap();

    for (i, &(diff_row, diff_col)) in DIFFS.iter().enumerate() {
        let (next_row, next_col) = (
            start_row.wrapping_add(diff_row),
            start_col.wrapping_add(diff_col),
        );

        if next_row >= h || next_col >= w || ccc[next_row][next_col] != '.' {
            continue;
        }

        que.push_back((next_row, next_col));
        visited[coord_to_idx((next_row, next_col))] = i + 1;
    }

    while let Some((cur_row, cur_col)) = que.pop_front() {
        let cur_cell_idx = coord_to_idx((cur_row, cur_col));

        for &(diff_row, diff_col) in &DIFFS {
            let (next_row, next_col) = (
                cur_row.wrapping_add(diff_row),
                cur_col.wrapping_add(diff_col),
            );

            if next_row >= h || next_col >= w || ccc[next_row][next_col] != '.' {
                continue;
            }

            let next_cell_idx = coord_to_idx((next_row, next_col));

            if visited[next_cell_idx] == 0 {
                visited[next_cell_idx] = visited[cur_cell_idx];
                que.push_back((next_row, next_col));
            } else if visited[next_cell_idx] != visited[cur_cell_idx] {
                return true;
            }
        }
    }

    return false;
}
