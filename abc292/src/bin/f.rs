use proconio::input;

fn main() {
    input! {
        (a, b): (f64, f64),
    }

    let mut left = 0.0;
    let mut right = std::f64::consts::PI / 6.0;

    let calc_length = |theta: f64| {
        let len1 = a / (std::f64::consts::PI / 6.0 - theta).cos();
        let len2 = b / theta.cos();

        len1.min(len2)
    };

    while right - left >= 1e-12 {
        let width = (right - left) / 3.0;

        let mid1 = left + width;
        let mid2 = right - width;

        let min_length_1 = calc_length(mid1);
        let min_length_2 = calc_length(mid2);

        if min_length_1 >= min_length_2 {
            right = mid2;
        } else {
            left = mid1;
        }
    }

    let ans = calc_length(left);
    println!("{}", ans);
}
