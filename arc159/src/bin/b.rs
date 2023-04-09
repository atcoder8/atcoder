use num::Integer;
use proconio::input;

fn main() {
    input! {
        (mut a, mut b): (usize, usize),
    }

    let mut ans = 0;

    if a < b {
        std::mem::swap(&mut a, &mut b);
    }

    while b != 0 {
        let r = a % b;
        if r == 0 {
            ans += 1;
            break;
        }
        a -= r;
        b -= r;
        let gcd = a.gcd(&b);
        a /= gcd;
        b /= gcd;

        ans += r / gcd;
    }

    println!("{}", ans);
}
