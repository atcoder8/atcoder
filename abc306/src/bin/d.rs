use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, i64); n],
    }

    let mut health = 0;
    let mut pain = 0;

    for &(x, y) in &xy {
        if x == 0 {
            health = health.max(health + y).max(pain + y);
        } else {
            pain = pain.max(health + y);
        }
    }

    println!("{}", health.max(pain));
}
