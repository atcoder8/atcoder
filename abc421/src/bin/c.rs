use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let a_positions = s.iter().positions(|&ch| ch == 'A').collect_vec();
    let b_positions = s.iter().positions(|&ch| ch == 'B').collect_vec();

    let min_cost =
        calc_min_cost(&a_positions, &b_positions).min(calc_min_cost(&b_positions, &a_positions));
    println!("{}", min_cost);
}

fn calc_min_cost(positions1: &[usize], positions2: &[usize]) -> usize {
    let n = positions1.len();
    (0..n)
        .map(|i| positions1[i].saturating_sub(2 * i) + positions2[i].saturating_sub(2 * i + 1))
        .sum()
}
