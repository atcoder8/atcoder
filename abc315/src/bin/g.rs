use proconio::input;

fn main() {
    input! {
        (n, a, b, c, x): (i128, i128, i128, i128, i128),
    }

    let (x0, y0, g) = ext_gcd(b, c);
    let (b0, c0) = (b / g, c / g);

    let mut ans = 0;
    for i in 1..=n.min((x + a - 1) / a - 1) {
        let rem = x - a * i;

        if rem % g != 0 {
            continue;
        }

        let (x1, y1) = (rem / g * x0, rem / g * y0);
        let low = ceil_div(1 - x1, c0).max(ceil_div(y1 - n, b0));
        let high = floor_div(n - x1, c0).min(floor_div(y1 - 1, b0));

        ans += (high - low + 1).max(0);
    }

    println!("{}", ans);
}

fn floor_div(a: i128, b: i128) -> i128 {
    match (a >= 0, b >= 0) {
        (true, true) => a / b,
        (true, false) => -ceil_div(a, -b),
        (false, true) => -ceil_div(-a, b),
        (false, false) => -a / -b,
    }
}

fn ceil_div(a: i128, b: i128) -> i128 {
    match (a >= 0, b >= 0) {
        (true, true) => (a + b - 1) / b,
        (true, false) => -floor_div(a, -b),
        (false, true) => -floor_div(-a, b),
        (false, false) => (-a + -b - 1) / -b,
    }
}


/// Returns a tuple of `(x, y)` and `gcd(a, b)` that satisfy `ax + by = gcd(a, b)` in that order.
///
/// The returned `x`, `y` and `gcd(a, b)` satisfy the following:
///   * Both `|x|` and `|y|` are less than or equal to `max(|a|, |b|)`.
///   * `gcd(a, b)` is non-negative.
///
/// # Examples
///
/// ```
/// # use atcoder8_library::number_theory::ext_gcd;
///
/// let (x, y, g) = ext_gcd(6, 10);
///
/// assert_eq!(g, 2);
/// assert_eq!(6 * x + 10 * y, 2);
/// assert!(x.abs() <= 10 && y.abs() <= 10);
/// ```
pub fn ext_gcd(mut a: i128, mut b: i128) -> (i128, i128, i128) {
    if a == 0 && b == 0 {
        return (0, 0, 0);
    }

    let (mut s, mut t, mut u, mut v) = (1, 0, 0, 1);
    while b != 0 {
        (a, b, s, t, u, v) = (b, a % b, t, s - a / b * t, v, u - a / b * v);
    }

    (a.signum() * s, a.signum() * u, a.abs())
}
