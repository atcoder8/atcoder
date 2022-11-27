use proconio::input;

fn calc_time(a: usize, b: usize, k: usize) -> f64 {
    a as f64 / (1.0 + k as f64).sqrt() + b as f64 * k as f64
}

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let mut left: usize = 0;
    let mut right: usize = 2e18 as usize;
    while left + 2 < right {
        let mid1 = left + (right - left) / 3;
        let mid2 = right - (right - left) / 3;
        if calc_time(a, b, mid1) < calc_time(a, b, mid2) {
            right = mid2;
        } else {
            left = mid1;
        }
    }

    let ans = (left..=right)
        .map(|i| calc_time(a, b, i))
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    println!("{}", ans);
}
