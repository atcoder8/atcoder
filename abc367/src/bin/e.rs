use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        xx: [Usize1; n],
        aa: [usize; n],
    }

    let mut doubling = vec![xx];
    for exp in 1_usize.. {
        if k >> exp == 0 {
            break;
        }

        let perm = &doubling[exp - 1];
        let double_perm = (0..n).map(|i| perm[perm[i]]).collect_vec();
        doubling.push(double_perm);
    }

    let mut permutation = (0..n).collect_vec();
    for exp in 0.. {
        if k >> exp == 0 {
            break;
        }

        if k >> exp & 1 == 1 {
            permutation.iter_mut().for_each(|x| *x = doubling[exp][*x]);
        }
    }

    let ans = permutation.iter().map(|&x| aa[x]).join(" ");
    println!("{}", ans);
}
