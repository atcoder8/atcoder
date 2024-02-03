use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = s.split('.').last().unwrap();
    println!("{}", ans);
}
