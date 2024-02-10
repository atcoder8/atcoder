use proconio::input;

fn main() {
    match solve() {
        Some((a, b)) => println!("{a} {b}"),
        None => println!("-1"),
    }
}

fn solve() -> Option<(i128, i128)> {
    input! {
        (x, y): (i128, i128),
    }

    if x == 0 {
        return if y.abs() <= 2 { Some((2 / y, y)) } else { None };
    }

    let (_u, mut v, g) = ext_gcd(x, -y);

    if g.abs() > 2 {
        return None;
    }

    if g.abs() == 1 {
        v *= 2;
    }

    if ((y * v + 2) / x).abs() > 10_i128.pow(18) {
        return None;
    }

    Some((v, (y * v + 2) / x))
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

    let sgn = a.signum();
    (sgn * s, sgn * u, sgn * a)
}
