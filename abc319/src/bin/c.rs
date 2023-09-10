use itertools::Itertools;
use proconio::input;

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn main() {
    input! {
        cc: [usize; 9],
    }

    let is_ok = |orders: &[usize]| {
        for line in LINES {
            if orders
                .iter()
                .filter(|&&order| line.contains(&order))
                .map(|&order| cc[order])
                .take(2)
                .all_equal()
            {
                return false;
            }
        }

        true
    };

    let cnt_ok = (0..9)
        .permutations(9)
        .filter(|orders| is_ok(orders))
        .count();
    println!("{}", cnt_ok as f64 / 362880.0);
}
