use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect::<String>();
    println!("{}", ans);
}
