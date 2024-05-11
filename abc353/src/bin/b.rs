use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let mut ans = 0;
    let mut cnt = 0;
    for &a in &aa {
        if cnt + a <= k {
            cnt += a;
        } else {
            ans += 1;
            cnt = a;
        }
    }
    ans += (cnt != 0) as usize;

    println!("{}", ans);
}
