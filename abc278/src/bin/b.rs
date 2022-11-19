use proconio::input;

fn main() {
    let (h, m) = solve();
    println!("{} {}", h, m);
}

fn solve() -> (usize, usize) {
    input! {
        (mut h, mut m): (usize, usize),
    }

    loop {
        let ch = h / 10 * 10 + m / 10;
        let cm = h % 10 * 10 + m % 10;

        if ch < 24 && cm < 60 {
            break (h, m);
        }

        m += 1;
        if m == 60 {
            h += 1;
            m = 0;
            if h == 24 {
                h = 0;
            }
        }
    }
}
