use proconio::input;

fn main() {
    input! {
        (n, s): (usize, usize),
        aa: [usize; n],
    }

    let mut terminals = vec![0; n];
    let mut sum = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && sum + aa[right] <= s {
            sum += aa[right];
            right += 1;
        }

        terminals[left] = right;

        sum -= aa[left];
    }

    let mut dp = vec![0; n + 1];
    for left in (0..n).rev() {
        dp[left] = dp[terminals[left]] + n - left;
    }

    let ans: usize = dp.iter().sum();
    println!("{}", ans);
}
