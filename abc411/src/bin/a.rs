use proconio::input;

fn main() {
    input! {
        p: String,
        l: usize,
    }

    let ans = if p.len() >= l { "Yes" } else { "No" };
    println!("{}", ans);
}
