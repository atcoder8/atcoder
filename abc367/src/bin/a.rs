use proconio::input;

fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    let (p, q, r) = (a < b, b < c, c < a);
    let ans = p & q || q & r || r & p;
    println!("{}", if ans { "Yes" } else { "No" });
}
