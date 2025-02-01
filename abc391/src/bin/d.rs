use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, w): (usize, usize),
        xy: [(Usize1, Usize1); n],
        q: usize,
        ta: [(usize, Usize1); q],
    }

    // 列ごとのブロック
    let mut blocks_by_columns = vec![vec![]; w];
    for (i, &(x, y)) in enumerate(&xy) {
        blocks_by_columns[x].push((i, y));
    }
    blocks_by_columns
        .iter_mut()
        .for_each(|blocks| blocks.sort_unstable_by_key(|&(_, y)| y));

    let num_disappear_lines = blocks_by_columns
        .iter()
        .map(|blocks| blocks.len())
        .min()
        .unwrap();

    let mut disappear_times: Vec<Option<usize>> = vec![None; n];
    for line in 0..num_disappear_lines {
        let disappear_time = blocks_by_columns
            .iter()
            .map(|blocks| blocks[line].1)
            .max()
            .unwrap()
            + 1;
        blocks_by_columns
            .iter()
            .for_each(|blocks| disappear_times[blocks[line].0] = Some(disappear_time));
    }

    let is_disappeared = |t: usize, a: usize| disappear_times[a].map_or(true, |time| time > t);

    let ans = ta
        .iter()
        .map(|&(t, a)| if is_disappeared(t, a) { "Yes" } else { "No" })
        .join("\n");
    println!("{}", ans);
}
