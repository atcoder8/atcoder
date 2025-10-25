use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let sum = aa.iter().sum::<usize>();
    let ans = aa.iter().any(|&a| sum == m + a);
    println!("{}", if ans { "Yes" } else { "No" });
}
