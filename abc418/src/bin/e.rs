use itertools::Itertools;
use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }

    let differences = xy
        .iter()
        .tuple_combinations()
        .map(|(&(x1, y1), &(x2, y2))| {
            let dx = x1 - x2;
            let dy = y1 - y2;
            match dx.cmp(&0) {
                std::cmp::Ordering::Less => (-dx, -dy),
                std::cmp::Ordering::Equal => (0, dy.abs()),
                std::cmp::Ordering::Greater => (dx, dy),
            }
        });

    let directions = differences.clone().map(|(dx, dy)| {
        let gcd = dx.gcd(&dy);
        (dx / gcd, dy / gcd)
    });

    let inclusive = calc_num_combinations(directions);
    let exclusive = calc_num_combinations(differences) / 2;
    println!("{}", inclusive - exclusive);
}

fn calc_num_combinations(vectors: impl Iterator<Item = (i64, i64)>) -> usize {
    vectors
        .sorted_unstable()
        .dedup_with_count()
        .map(|(cnt, _)| cnt * (cnt - 1) / 2)
        .sum()
}
