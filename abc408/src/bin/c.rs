use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        lr: [(Usize1, usize); m],
    }

    let mut imos = vec![0_i64; n + 1];
    for &(l, r) in &lr {
        imos[l] += 1;
        imos[r] -= 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    let ans = *imos[..n].iter().min().unwrap();
    println!("{}", ans);
}
