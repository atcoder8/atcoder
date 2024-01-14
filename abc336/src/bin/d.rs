use itertools::izip;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut left_levels = vec![0; n];
    left_levels[0] = 1;
    for i in 1..n {
        left_levels[i] = (left_levels[i - 1] + 1).min(aa[i]);
    }

    let mut right_levels = vec![0; n];
    right_levels[n - 1] = 1;
    for i in (0..n - 1).rev() {
        right_levels[i] = (right_levels[i + 1] + 1).min(aa[i]);
    }

    let ans = izip!(left_levels, right_levels)
        .map(|(x, y)| x.min(y))
        .max()
        .unwrap();
    println!("{}", ans);
}
