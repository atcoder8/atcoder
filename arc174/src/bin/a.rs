use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, c): (usize, i64),
        aa: [i64; n],
    }

    let sum_a = aa.iter().sum::<i64>();

    let mut prefix_sum = vec![0; n + 1];
    for (i, &a) in enumerate(&aa) {
        prefix_sum[i + 1] = prefix_sum[i] + a;
    }

    let ans = if c >= 1 {
        let mut acc_min = prefix_sum.clone();
        for i in 0..n {
            acc_min[i + 1] = acc_min[i + 1].min(acc_min[i]);
        }

        let mut acc_max = prefix_sum.clone();
        for i in (0..n).rev() {
            acc_max[i] = acc_max[i].max(acc_max[i + 1]);
        }

        let max_diff = (0..=n).map(|i| acc_max[i] - acc_min[i]).max().unwrap();

        sum_a + max_diff * (c - 1)
    } else {
        let mut acc_max = prefix_sum.clone();
        for i in 0..n {
            acc_max[i + 1] = acc_max[i + 1].max(acc_max[i]);
        }

        let mut acc_min = prefix_sum.clone();
        for i in (0..n).rev() {
            acc_min[i] = acc_min[i].min(acc_min[i + 1]);
        }

        let min_diff = (0..=n).map(|i| acc_min[i] - acc_max[i]).min().unwrap();

        sum_a + min_diff * (c - 1)
    };

    println!("{}", ans);
}
