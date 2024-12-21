use std::collections::BTreeSet;

use im_rc::HashMap;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m, sx, sy): (usize, usize, i64, i64),
        xy: [(i64, i64); n],
        dc: [(char, i64); m],
    }

    let mut vertical = HashMap::<i64, BTreeSet<i64>>::new();
    let mut horizontal = HashMap::<i64, BTreeSet<i64>>::new();
    for &(x, y) in &xy {
        vertical.entry(x).or_default().insert(y);
        horizontal.entry(y).or_default().insert(x);
    }

    let mut count_houses = 0_usize;
    let mut cur_coord = (sx, sy);
    for &(d, c) in &dc {
        let diff = calc_diff(d, c);
        let next_coord = (cur_coord.0 + diff.0, cur_coord.1 + diff.1);

        let x_range = cur_coord.0.min(next_coord.0)..cur_coord.0.max(next_coord.0) + 1;
        let y_range = cur_coord.1.min(next_coord.1)..cur_coord.1.max(next_coord.1) + 1;

        let mut houses = vertical.remove(&cur_coord.0).unwrap_or_default();
        let extracted_houses = houses.range(y_range).cloned().collect_vec();
        count_houses += extracted_houses.len();
        for &y in &extracted_houses {
            houses.remove(&y);
            horizontal[&y].remove(&cur_coord.0);
        }
        vertical.insert(cur_coord.0, houses);

        let mut houses = horizontal.remove(&cur_coord.1).unwrap_or_default();
        let extracted_houses = houses.range(x_range).cloned().collect_vec();
        count_houses += extracted_houses.len();
        for &x in &extracted_houses {
            houses.remove(&x);
            vertical[&x].remove(&cur_coord.1);
        }
        horizontal.insert(cur_coord.1, houses);

        cur_coord = next_coord;
    }

    println!("{} {} {}", cur_coord.0, cur_coord.1, count_houses);
}

fn calc_diff(dir: char, c: i64) -> (i64, i64) {
    match dir {
        'U' => (0, c),
        'D' => (0, -c),
        'L' => (-c, 0),
        'R' => (c, 0),
        _ => panic!(),
    }
}
