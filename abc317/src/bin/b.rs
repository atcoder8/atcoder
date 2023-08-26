use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable();
    let ans = (0..n).find(|&i| aa[0] + i != aa[i]).unwrap_or(n) + aa[0];
    println!("{}", ans);
}
