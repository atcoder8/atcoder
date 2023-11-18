use proconio::input;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let ans = *aa.iter().rev().find(|&&a| a != aa[n - 1]).unwrap();
    println!("{}", ans);
}
