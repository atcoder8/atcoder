use proconio::input;

fn main() {
    input! {
        (n, a): (usize, usize),
        tt: [usize; n],
    }

    let mut time = 0;
    for &t in &tt {
        time = time.max(t) + a;
        println!("{}", time);
    }
}
