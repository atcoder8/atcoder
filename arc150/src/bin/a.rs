use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        println!("{}", if solve() { "Yes" } else { "No" });
    }
}

fn solve() -> bool {
    input! {
        (n, k): (usize, usize),
        s: Chars,
    }

    if !s.contains(&'1') {
        let mut blank_cnt = 0_usize;
        let mut flag = false;

        for &c in &s {
            if c == '?' {
                blank_cnt += 1;

                if blank_cnt == k {
                    if flag {
                        return false;
                    }

                    flag = true;
                } else if blank_cnt > k {
                    return false;
                }
            } else {
                blank_cnt = 0;
            }
        }

        return flag;
    }

    let left = s.iter().position(|&c| c == '1').unwrap();
    let right = n - s.iter().rev().position(|&c| c == '1').unwrap();

    if right - left > k {
        return false;
    }

    if s[left..right].iter().any(|&c| c == '0') {
        return false;
    }

    let rem = k - (right - left);

    if rem == 0 {
        return true;
    }

    let left_blank_cnt = s[..left].iter().rev().take_while(|&&c| c == '?').count();
    let right_blank_cnt = s[right..].iter().take_while(|&&c| c == '?').count();

    left_blank_cnt + right_blank_cnt == rem
        || (left_blank_cnt == 0 && right_blank_cnt >= rem)
        || (left_blank_cnt >= rem && right_blank_cnt == 0)
}
