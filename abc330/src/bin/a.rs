use proconio::input;

fn main() {
    input! {
        (n, l): (usize, usize),
        aa: [usize; n],
    }

    let ans = aa.iter().filter(|&&a| a >= l).count();
    println!("{}", ans);
}
