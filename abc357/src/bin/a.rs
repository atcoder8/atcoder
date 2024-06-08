use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        hh: [usize; n],
    }

    let mut ans = 0;
    let mut rem = m;
    for &h in &hh {
        if rem >= h {
            ans += 1;
            rem -= h;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
