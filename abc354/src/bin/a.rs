use proconio::input;

fn main() {
    input! {
        h: usize,
    }

    let ans = (1..).find(|&day| h + 1 >> day == 0).unwrap();
    println!("{}", ans);
}
