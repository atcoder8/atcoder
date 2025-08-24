use proconio::input;

fn main() {
    input! {
        _n: usize,
        t: String,
    }

    let mut ans = 0_usize;
    let mut parity = 0;
    let mut counts = [1, 0];
    for ch in t.chars() {
        if ch == '0' {
            parity = 1 - parity;
        }

        let cnt = &mut counts[parity];
        ans += *cnt;
        *cnt += 1;
    }
    println!("{}", ans);
}
