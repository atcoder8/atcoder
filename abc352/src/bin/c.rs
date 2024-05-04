use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let ans =
        ab.iter().map(|&(a, _)| a).sum::<usize>() + ab.iter().map(|&(a, b)| b - a).max().unwrap();
    println!("{}", ans);
}
