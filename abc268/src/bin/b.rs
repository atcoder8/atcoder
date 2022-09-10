use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let flag = s.len() <= t.len() && s[..] == t[..s.len()];
    println!("{}", if flag { "Yes" } else { "No" });
}
