use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let ans = s.replace("na", "nya");

    println!("{}", ans);
}
