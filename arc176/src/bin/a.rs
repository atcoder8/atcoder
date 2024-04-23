use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, Usize1); m],
    }

    let mut flags = vec![false; n];
    for &(a, b) in &ab {
        flags[(n + b - a) % n] = true;
    }

    let mut shortage = m - flags.iter().filter(|&&flag| flag).count();
    for i in 0..n {
        if shortage == 0 {
            break;
        }

        if !flags[i] {
            flags[i] = true;
            shortage -= 1;
        }
    }

    let coords = (0..n)
        .filter(|&start| flags[start])
        .flat_map(|start| (0..n).map(move |row| (row, (start + row) % n)));

    let ans = coords
        .map(|(row, col)| format!("{} {}", row + 1, col + 1))
        .join("\n");
    println!("{}\n{}", n * m, ans);
}
