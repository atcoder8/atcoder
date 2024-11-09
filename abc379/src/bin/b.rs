use proconio::input;

fn main() {
    input! {
        (_n, k): (usize, usize),
        s: String,
    }

    let mut ans = 0_usize;
    let mut cnt = 0_usize;
    for c in s.chars() {
        if c == 'O' {
            cnt += 1;

            if cnt == k {
                ans += 1;
                cnt = 0;
            }
        } else {
            cnt = 0;
        }
    }
    println!("{}", ans);
}
