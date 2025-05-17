use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    let ans = a > c || (a == c && b > d);
    println!("{}", if ans { "Yes" } else { "No" });
}
