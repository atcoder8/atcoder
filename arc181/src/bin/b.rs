use itertools::Itertools;
use num::Integer;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    let ans = (0..t)
        .map(|_| if solve() { "Yes" } else { "No" })
        .join("\n");
    println!("{}", ans);
}

fn solve() -> bool {
    input! {
        s: String,
        x: String,
        y: String,
    }

    let zero_num_x = x.chars().filter(|&c| c == '0').count();
    let one_num_x = x.len() - zero_num_x;
    let zero_num_y = y.chars().filter(|&c| c == '0').count();
    let one_num_y = y.len() - zero_num_y;

    let diff_zero_num = zero_num_x as isize - zero_num_y as isize;
    let diff_one_num = one_num_x as isize - one_num_y as isize;

    if diff_one_num == 0 {
        return diff_zero_num == 0;
    }

    let numer = -(diff_zero_num * s.len() as isize);
    let denom = diff_one_num;

    if numer % denom != 0 || numer / denom < 0 {
        return false;
    }

    let t_len = (numer / denom) as usize;

    let gcd = s.len().gcd(&t_len);

    s[..gcd].repeat(s.len() / gcd) == s
}
