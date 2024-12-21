use ndarray::prelude::*;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (h, w, x, y): (usize, usize, Usize1, Usize1),
        sss: [Chars; h],
        t: String,
    }

    let mut ans = 0_u32;
    let mut visited = Array2::from_elem((h, w), false);
    let mut cur_coord = (x, y);
    for dir in t.chars() {
        let diff = dir_to_diff(dir);
        let adj_coord = (
            cur_coord.0.wrapping_add(diff.0),
            cur_coord.1.wrapping_add(diff.1),
        );
        let c = sss[adj_coord.0][adj_coord.1];
        if c != '#' {
            cur_coord = adj_coord;
            if c == '@' && !visited[adj_coord] {
                ans += 1;
            }
            visited[adj_coord] = true;
        }
    }

    println!("{} {} {}", cur_coord.0 + 1, cur_coord.1 + 1, ans);
}

fn dir_to_diff(dir: char) -> (usize, usize) {
    match dir {
        'U' => (!0, 0),
        'D' => (1, 0),
        'L' => (0, !0),
        'R' => (0, 1),
        _ => panic!(),
    }
}
