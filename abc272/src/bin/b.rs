use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
    }

    let mut same = vec![vec![false; n]; n];
    for i in 0..n {
        same[i][i] = true;
    }

    for _ in 0..m {
        input! {
            k: usize,
            xx: [Usize1; k],
        }

        for i in 0..k {
            for j in 0..k {
                same[xx[i]][xx[j]] = true;
            }
        }
    }

    let flag = same.into_iter().flatten().all(|x| x);
    println!("{}", if flag { "Yes" } else { "No" });
}
