use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let is_ok = |x: usize| aa.iter().filter(|&&a| a >= x).count() >= x;

    let ans = (0..=n).rev().find(|&a| is_ok(a)).unwrap();
    println!("{}", ans);
}
