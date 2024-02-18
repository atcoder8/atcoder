use std::{cmp::Reverse, collections::BTreeMap};

use maplit::btreemap;
use proconio::input;

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        (h, w, n): (usize, usize, usize),
        mut aa: [u32; n],
    }

    let mut squares = cut_into_squares((h, w));

    aa.sort_unstable_by_key(|&a| Reverse(a));

    for &a in &aa {
        let Some((max_log, max_cnt)) = squares.pop_last() else { return false; };

        if max_log < a {
            return false;
        }

        if max_cnt - 1 != 0 {
            squares.insert(max_log, max_cnt - 1);
        }

        let max_size = 1 << max_log;
        let cut_size = 1 << a;

        for (size, cnt) in cut_into_squares((max_size - cut_size, max_size)) {
            *squares.entry(size).or_default() += cnt;
        }

        for (size, cnt) in cut_into_squares((cut_size, max_size - cut_size)) {
            *squares.entry(size).or_default() += cnt;
        }
    }

    true
}

/// 一辺の長さが2の冪乗である最大の正方形を切り出すことを繰り返し、切り出された正方形のサイズごとにlog2(正方形のサイズ)とその個数をペアにしたMapを返します。
fn cut_into_squares(shape: (usize, usize)) -> BTreeMap<u32, usize> {
    let (mut h, mut w) = shape;
    if h > w {
        std::mem::swap(&mut h, &mut w);
    }

    if h == 0 {
        return btreemap! {};
    }

    let floor_log2_h = (0_u32..).find(|&i| h >> i == 1).unwrap();

    let square_size = 1 << floor_log2_h;
    let square_num = w / square_size;

    let mut squares = btreemap! {floor_log2_h => square_num};

    let mut rem_h = h - square_size;
    let mut rem_w = w % square_size;

    if rem_h < rem_w {
        std::mem::swap(&mut h, &mut w);
        std::mem::swap(&mut rem_h, &mut rem_w);
    }

    for (size, cnt) in cut_into_squares((rem_h, w)) {
        *squares.entry(size).or_default() += cnt;
    }

    for (size, cnt) in cut_into_squares((h - rem_h, rem_w)) {
        *squares.entry(size).or_default() += cnt;
    }

    squares
}
