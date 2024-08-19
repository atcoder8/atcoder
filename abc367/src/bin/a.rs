use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let (p, q, r) = (a < b, b < c, c < a);
    println!("{}", if !(p ^ q ^ r) { "Yes" } else { "No" });
}
