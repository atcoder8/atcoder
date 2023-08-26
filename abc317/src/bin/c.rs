use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut mat = vec![vec![None; n]; n];
    for &(a, b, c) in &abc {
        mat[a][b] = Some(c);
        mat[b][a] = Some(c);
    }

    let calc_sum_dist = |perm: &Vec<usize>| {
        let mut sum_dist = 0;
        for window in perm.windows(2) {
            if let Some(dist) = mat[window[0]][window[1]] {
                sum_dist += dist;
            } else {
                return sum_dist;
            }
        }

        sum_dist
    };

    let mut perm = (0..n).collect_vec();
    let mut ans = calc_sum_dist(&perm);
    while perm.next_permutation() {
        ans = ans.max(calc_sum_dist(&perm));
    }

    println!("{}", ans);
}
