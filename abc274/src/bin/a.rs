use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let mut ans = b * 10000 / a;
    if ans % 10 >= 5 {
        ans += 10;
    }
    ans /= 10;

    println!("{}.{:03}", ans / 1000, ans % 1000);
}
