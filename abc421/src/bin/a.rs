use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
        (x, y): (usize, String),
    }

    println!("{}", if ss[x - 1] == y { "Yes" } else { "No" });
}
