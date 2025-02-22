use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ss: [String; n],
    }

    ss.sort_unstable_by_key(|s| s.len());
    println!("{}", ss.join(""));
}
