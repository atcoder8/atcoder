use proconio::input;

fn main() {
    input! {
        (n, h, x): (usize, usize, usize),
        pp: [usize; n],
    }

    let ans = pp.iter().position(|&p| h + p >= x).unwrap() + 1;
    println!("{}", ans);
}
