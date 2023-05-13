use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut s_counts = vec![0_usize; 26];
    let mut s_at_cnt = 0_usize;
    for &c in &s {
        if c == '@' {
            s_at_cnt += 1;
        } else {
            s_counts[(c as u8 - b'a') as usize] += 1;
        }
    }

    let mut t_counts = vec![0_usize; 26];
    let mut t_at_cnt = 0_usize;
    for &c in &t {
        if c == '@' {
            t_at_cnt += 1;
        } else {
            t_counts[(c as u8 - b'a') as usize] += 1;
        }
    }

    let str = "atcoder";

    for (i, (&s_cnt, &t_cnt)) in s_counts.iter().zip(&t_counts).enumerate() {
        if s_cnt != t_cnt && !str.contains((i as u8 + b'a') as char) {
            return false;
        }

        if s_cnt <= t_cnt {
            let diff = t_cnt - s_cnt;

            if s_at_cnt < diff {
                return false;
            }

            s_at_cnt -= diff;
        } else {
            let diff = s_cnt - t_cnt;

            if t_at_cnt < diff {
                return false;
            }

            t_at_cnt -= diff;
        }
    }

    true
}
