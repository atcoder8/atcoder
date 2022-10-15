use proconio::input;

fn main() {
    input! {
        (mut x, k): (usize, usize),
    }

    let mut ten = 1;
    for _ in 0..k {
        let d = x % (10 * ten) / ten;
        if d <= 4 {
            x -= d * ten;
        } else {
            x += (10 - d) * ten;
        }
        ten *= 10;
    }

    println!("{}", x);
}
