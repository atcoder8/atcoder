use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let sum: usize = aa.iter().sum();
    let (low, high) = (sum / n, (sum + n - 1) / n);

    let mut add = 0;
    let mut sub = 0;
    for &a in &aa {
        if a < low {
            add += low - a;
        } else if a > high {
            sub += a - high;
        }
    }

    println!("{}", add.max(sub));
}
