use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = a % 3 != 0 && b == a + 1;
    println!("{}", if ans { "Yes" } else { "No" });
}
