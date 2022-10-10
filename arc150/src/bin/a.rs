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

    let all_one_cnt = s.iter().filter(|&&c| c == '1').count();

    let mut zero_cnt = 0;
    let mut one_cnt = 0;

    for &c in s.iter().take(k) {
        if c == '0' {
            zero_cnt += 1;
        } else if c == '1' {
            one_cnt += 1;
        }
    }

    if one_cnt == all_one_cnt && zero_cnt == 0 {}

    (one_cnt == all_one_cnt && zero_cnt == 0) as usize
        + (k..n)
            .filter(|&i| {
                if s[i - k] == '0' {
                    zero_cnt -= 1;
                } else if s[i - k] == '1' {
                    one_cnt -= 1;
                }

                if s[i] == '0' {
                    zero_cnt += 1;
                } else if s[i] == '1' {
                    one_cnt += 1;
                }

                one_cnt == all_one_cnt && zero_cnt == 0
            })
            .count()
        == 1
}
