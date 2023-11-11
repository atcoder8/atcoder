use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        ss: [usize; n],
    }

    let ans: usize = ss.iter().filter(|&&s| s <= x).sum();
    println!("{}", ans);
}
