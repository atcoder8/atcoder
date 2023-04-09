use proconio::input;

fn main() {
    input! {
        (mut a, mut b): (usize, usize),
    }

    let mut ans = 0;
    while a != b {
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }

        let cnt = (a - 1) / b;
        ans += cnt;
        a -= cnt * b;
    }

    println!("{}", ans);
}
