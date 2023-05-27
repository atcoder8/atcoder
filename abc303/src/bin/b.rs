use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aaa: [[Usize1; n]; m],
    }

    let mut adjacent = vec![vec![false; n]; n];
    for aa in &aaa {
        for i in 0..(n - 1) {
            adjacent[aa[i]][aa[i + 1]] = true;
            adjacent[aa[i + 1]][aa[i]] = true;
        }
    }

    let mut ans = 0_usize;
    for i in 0..n {
        for j in (i + 1)..n {
            if !adjacent[i][j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
