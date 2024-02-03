use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut cur = 0;
    let mut min = 0;
    for &a in &aa {
        cur += a;
        min = min.min(cur);
    }

    println!("{}", cur - min);
}
