use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut ans = 0_usize;
    let mut sum = aa.iter().sum::<usize>();
    for &a in &aa {
        sum -= a;
        ans += a * sum;
    }
    println!("{}", ans);
}
