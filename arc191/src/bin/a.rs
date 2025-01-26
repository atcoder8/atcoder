// Tの各桁を降順に見る
// 数字を増やせる桁があるならなるべく上位の桁を変更する
// どの桁を選んでも減るならなるべく下位の桁を変更する
// 増やせないが同じ値の桁があるならその桁を選ぶ

use std::{cmp::Reverse, collections::BTreeSet};

use amplify::confinement::Collection;
use itertools::{enumerate, Itertools};
use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> String {
    input! {
        (n, m): (usize, usize),
        s: String,
        t: String,
    }

    let mut positions_by_value: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); 10];
    for (i, ch) in enumerate(s.chars()) {
        let value = ch.to_digit(10).unwrap() as usize;
        positions_by_value[value].push(i);
    }

    let replacers = enumerate(t.chars())
        .map(|(i, ch)| Replacer::new(i, ch.to_digit(10).unwrap() as usize))
        .sorted_unstable_by_key(|replacer| (Reverse(replacer.value), replacer.position))
        .collect_vec();

    let mut edited = s
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect_vec();

    for &replacer in &replacers {
        if let Some(v) = (0..replacer.value)
            .filter(|&v| !positions_by_value[v].is_empty())
            .min_by_key(|&v| positions_by_value[v].first().unwrap())
        {
            let pos = positions_by_value[v].pop_first().unwrap();
            edited[pos] = replacer.value;
            positions_by_value[replacer.value].insert(pos);
            continue;
        }

        if replacer.position != m - 1 || !positions_by_value[replacer.value].is_empty() {
            continue;
        }

        positions_by_value[edited[n - 1]].remove(&(n - 1));
        edited[n - 1] = replacer.value;
        positions_by_value[replacer.value].push(n - 1);
    }

    edited.iter().join("")
}

#[derive(Debug, Clone, Copy)]
struct Replacer {
    position: usize,
    value: usize,
}

impl Replacer {
    fn new(position: usize, value: usize) -> Self {
        Self { position, value }
    }
}
