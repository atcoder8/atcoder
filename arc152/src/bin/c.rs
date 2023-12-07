use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let d = aa[n - 1] - aa[0];
    let ans = aa[0] % aa.iter().fold(d, |acc, &a| acc.gcd(&(2 * (a - aa[0])))) + d;
    println!("{}", ans);
}
