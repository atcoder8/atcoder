use proconio::input;

fn main() {
    input! {
        (n, l, r): (usize, usize, usize),
        xy: [(usize, usize); n],
    }

    let ans = xy.iter().filter(|&&(x, y)| x <= l && r <= y).count();
    println!("{}", ans);
}
