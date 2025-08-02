use proconio::input;

fn main() {
    input! {
        (_n, a, b): (usize, usize, usize),
        s: String,
    }

    let ans = &s[a..s.len() - b];
    println!("{}", ans);
}
