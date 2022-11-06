use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Chars};

const DIFFS: [(usize, usize); 4] = [(!0, 0), (0, !0), (0, 1), (1, 0)];

fn main() {
    input! {
        (h, _w): (usize, usize),
        ccc: [Chars; h],
    }

    println!(
        "{}",
        if solve_by_union_find(&ccc) {
            "Yes"
        } else {
            "No"
        }
    );
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

fn solve_by_union_find(ccc: &Vec<Vec<char>>) -> bool {
    let (h, w) = (ccc.len(), ccc[0].len());

    let coord_to_idx = |(i, j)| i * w + j;

    let mut uf: UnionFind<usize> = UnionFind::new(h * w);

    for i in 0..h {
        for j in 0..w {
            if ccc[i][j] != '.' {
                continue;
            }

            if i < h - 1 && ccc[i + 1][j] == '.' {
                uf.union(coord_to_idx((i, j)), coord_to_idx((i + 1, j)));
            }

            if j < w - 1 && ccc[i][j + 1] == '.' {
                uf.union(coord_to_idx((i, j)), coord_to_idx((i, j + 1)));
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
