use proconio::input;

fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = a.pow(b as u32) + b.pow(a as u32);
    println!("{}", ans);
}
