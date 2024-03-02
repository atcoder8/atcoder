use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = (0..10).find(|&i| i != a + b).unwrap();
    println!("{}", ans);
}
