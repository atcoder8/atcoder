use itertools::{iproduct, Itertools};
use ndarray::Array2;
use proconio::input;

const MAX: usize = 5010;

fn main() {
    input! {
        (t, m): (usize, usize),
        ccc: [[usize]; t],
    }

    let mut comb_array = Array2::from_elem((MAX + 1, MAX + 1), 0_usize);
    for j in 0..=MAX {
        comb_array[(j, 0)] = 1;
    }
    comb_array[(0, 0)] = 1;
    for (i, j) in iproduct!(1..=MAX, 1..=MAX) {
        comb_array[(i, j)] = (comb_array[(i - 1, j - 1)] + comb_array[(i - 1, j)]) % m;
    }

    let mut fac = vec![1; MAX + 1];
    for i in 1..=MAX {
        fac[i] = i * (fac[i - 1]) % m;
    }

    let solve = |cc: &[usize]| {
        let sum_c = cc.iter().sum::<usize>();
        let mut rem = sum_c;
        let mut ans = 1_usize;
        for &c in cc {
            ans = ans * comb_array[(rem, c)] % m;
            rem -= c;
        }

        ans
    };

    println!("{}", ccc.iter().map(|cc| solve(cc)).join("\n"));
}
