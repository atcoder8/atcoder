use itertools::join;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        ccc: [Chars; h],
    }

    let n = h.min(w);
    let mut ss = vec![0; n + 1];

    for i in 0..h {
        for j in 0..w {
            if ccc[i][j] == '.' {
                continue;
            }

            for d in 1..=(n + 1) {
                let upper_left = i >= d && j >= d && ccc[i - d][j - d] == '#';
                let upper_right = i >= d && j + d < w && ccc[i - d][j + d] == '#';
                let lower_left = i + d < h && j >= d && ccc[i + d][j - d] == '#';
                let lower_right = i + d < h && j + d < w && ccc[i + d][j + d] == '#';

                if !upper_left || !upper_right || !lower_left || !lower_right {
                    ss[d - 1] += 1;

                    break;
                }
            }
        }
    }

    println!("{}", join(&ss[1..], " "));
}
