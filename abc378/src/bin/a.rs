use proconio::input;

fn main() {
    input! {
        mut aa: [usize; 4],
    }

    aa.sort_unstable();

    let mut ans = 0_usize;
    let mut hold = None;
    for &a in &aa {
        match hold.take() {
            Some(prev) if prev == a => ans += 1,
            _ => hold = Some(a),
        }
    }
    println!("{}", ans);
}
