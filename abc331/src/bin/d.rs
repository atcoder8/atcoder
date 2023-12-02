use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, q): (usize, usize),
        colors: [Chars; n],
        queries: [(usize, usize, usize, usize); q],
    }

    let mut acc = vec![vec![0_usize; n + 1]; n + 1];
    for (i, j) in iproduct!(0..n, 0..n) {
        acc[i + 1][j + 1] = (colors[i][j] == 'B') as usize;
    }
    for (i, j) in iproduct!(0..=n, 0..n) {
        acc[i][j + 1] += acc[i][j];
    }
    for (i, j) in iproduct!(0..n, 0..=n) {
        acc[i + 1][j] += acc[i][j];
    }

    let inner_count_black = |bottom: usize, right: usize| {
        let (qh, rh) = (bottom / n, bottom % n);
        let (qw, rw) = (right / n, right % n);

        qh * qw * acc[n][n] + acc[rh][n] * qw + acc[n][rw] * qh + acc[rh][rw]
    };

    let count_black = |top: usize, bottom: usize, left: usize, right: usize| {
        inner_count_black(bottom, right) + inner_count_black(top, left)
            - (inner_count_black(top, right) + inner_count_black(bottom, left))
    };

    for (top, left, bottom, right) in queries {
        println!("{}", count_black(top, bottom + 1, left, right + 1));
    }
}
