use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(Usize1, char); m],
    }

    let mut born = vec![false; n];
    let ans = ab
        .iter()
        .map(|&(a, b)| {
            if !born[a] && b == 'M' {
                born[a] = true;
                "Yes"
            } else {
                "No"
            }
        })
        .join("\n");
    println!("{}", ans);
}
