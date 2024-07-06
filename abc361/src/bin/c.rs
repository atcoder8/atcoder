use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let ans = (0..=k)
        .map(|left| aa[left + n - k - 1] - aa[left])
        .min()
        .unwrap();
    println!("{}", ans);
}
