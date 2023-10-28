use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    let ans = (x < y && y - x <= 2) || (x > y && x - y <= 3);
    println!("{}", if ans { "Yes" } else { "No" });
}
