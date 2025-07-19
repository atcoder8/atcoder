use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        x: usize,
    }

    let ans = aa.contains(&x);
    println!("{}", if ans { "Yes" } else { "No" });
}
