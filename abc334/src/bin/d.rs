use itertools::enumerate;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, q): (usize, usize),
        mut rr: [usize; n],
        xx: [usize; q],
    }

    rr.sort_unstable();

    let mut prefix_sum = vec![0; n + 1];
    for (i, &r) in enumerate(&rr) {
        prefix_sum[i + 1] = prefix_sum[i] + r;
    }

    for &x in &xx {
        let ans = prefix_sum.upper_bound(&x) - 1;
        println!("{}", ans);
    }
}
