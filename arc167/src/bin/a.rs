use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let mut ans = 0;
    for i in 0..(n - m) {
        ans += (aa[i] + aa[2 * (n - m) - 1 - i]).pow(2);
    }
    ans += aa[2 * (n - m)..].iter().map(|&a| a.pow(2)).sum::<usize>();

    println!("{}", ans);
}
