use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let for_num = ss.iter().filter(|&s| s == "For").count();

    println!("{}", if for_num > n / 2 { "Yes" } else { "No" });
}
