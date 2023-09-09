use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec!['-'; n + 1];
    for j in 1..=9 {
        if n % j != 0 {
            continue;
        }

        for i in (0..=n).step_by(n / j) {
            if ans[i] == '-' {
                ans[i] = (j as u8 + b'0') as char;
            }
        }
    }

    let ans: String = ans.iter().collect();
    println!("{}", ans);
}
