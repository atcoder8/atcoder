use proconio::input;

fn main() {
    input! {
        (x, k): (usize, u32),
    }

    let mut ans = x;
    for _ in 0..k {
        ans = (ans + 5) / 10;
    }
    ans *= 10_usize.pow(k);

    println!("{}", ans);
}
