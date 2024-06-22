use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let ans = ss.iter().filter(|&s| s == "Takahashi").count();
    println!("{}", ans);
}
