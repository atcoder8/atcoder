use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut init_counts = vec![0; n];
    let mut coords = ab.clone();
    for &(_a, b) in &ab {
        init_counts[b] += 1;
    }
    let mut set: BTreeSet<(usize, usize)> = (0..n).map(|col| (0, col)).collect();

    for row in 0..n {
        let rem = m - ab.iter().filter(|&&(a, _b)| a == row).count();
        let bb = ab
            .iter()
            .filter(|&&(a, _b)| a == row)
            .map(|&(_a, b)| b)
            .collect_vec();
        let small = set
            .iter()
            .filter(|&&(cnt, col)| !bb.contains(&col) && cnt + init_counts[col] < m)
            .take(rem)
            .cloned()
            .collect_vec();
        for &(cnt, col) in &small {
            coords.push((row, col));

            set.remove(&(cnt, col));
            set.insert((cnt + 1, col));
        }
    }

    coords.sort_unstable();

    println!(
        "{}\n{}",
        coords.len(),
        coords
            .iter()
            .map(|(row, col)| format!("{} {}", row + 1, col + 1))
            .join("\n")
    );
}
