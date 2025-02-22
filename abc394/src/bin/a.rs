use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s.chars().filter(|&c| c == '2').collect::<String>();
    println!("{}", ans);
}
