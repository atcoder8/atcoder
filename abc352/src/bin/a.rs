use proconio::input;

fn main() {
    input! {
        _n: usize,
        mut x: usize,
        mut y: usize,
        z: usize,
    }

    if x > y {
        std::mem::swap(&mut x, &mut y);
    }

    let ans = x < z && z < y;
    println!("{}", if ans { "Yes" } else { "No" });
}
