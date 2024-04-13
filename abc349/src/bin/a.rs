use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n - 1],
    }

    let ans = -aa.iter().sum::<i64>();
    println!("{}", ans);
}
