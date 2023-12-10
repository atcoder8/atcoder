use proconio::{input, marker::Chars};

fn main() {
    input! {
        (_n, m): (usize, usize),
        s: Chars,
    }

    let mut ans = 0;
    let mut washed_plain = m;
    let mut washed_logo = 0;
    let mut used_plain = 0;
    let mut used_logo = 0;
    for &c in &s {
        match c {
            '0' => {
                washed_plain += used_plain;
                used_plain = 0;
                washed_logo += used_logo;
                used_logo = 0;
            }

            '1' => {
                if washed_plain != 0 {
                    washed_plain -= 1;
                    used_plain += 1;
                } else if washed_logo != 0 {
                    washed_logo -= 1;
                    used_logo += 1;
                } else {
                    ans += 1;
                    used_logo += 1;
                }
            }

            '2' => {
                if washed_logo != 0 {
                    washed_logo -= 1;
                    used_logo += 1;
                } else {
                    ans += 1;
                    used_logo += 1;
                }
            }

            _ => panic!(),
        }
    }

    println!("{}", ans);
}
