use std::collections::HashMap;

use itertools::join;
use proconio::input;

fn main() {
    input! {
        (grid_h, grid_w, _n, paint_h, paint_w): (usize, usize, usize, usize, usize),
        aaa: [[usize; grid_w]; grid_h],
    }

    let mut ans = vec![vec![0; grid_w - paint_w + 1]; grid_h - paint_h + 1];

    for top in 0..=(grid_h - paint_h) {
        let bottom = top + paint_h;

        let mut map = HashMap::new();

        for i in 0..grid_h {
            for j in 0..grid_w {
                if !(top <= i && i < bottom && j < paint_w) {
                    *map.entry(aaa[i][j]).or_insert(0) += 1;
                }
            }
        }

        ans[top][0] = map.len();

        for left in 0..(grid_w - paint_w) {
            let right = left + paint_w;

            for i in top..bottom {
                *map.get_mut(&aaa[i][right]).unwrap() -= 1;
                if *map.get(&aaa[i][right]).unwrap() == 0 {
                    map.remove(&aaa[i][right]);
                }
            }

            for i in top..bottom {
                *map.entry(aaa[i][left]).or_insert(0) += 1;
            }

            ans[top][left + 1] = map.len();
        }
    }

    for row in ans {
        println!("{}", join(row, " "));
    }
}
