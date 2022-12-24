use proconio::{input, marker::Chars};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        s: Chars,
    }

    let mut t: Vec<char> = vec![];
    let mut exists = vec![false; 26];

    for &c in &s {
        if c == '(' {
        } else if c == ')' {
            while let Some(last_c) = t.pop() {
                if last_c == '(' {
                    break;
                }

                exists[(last_c as u8 - 'a' as u8) as usize] = false;
            }
        } else {
            if exists[(c as u8 - 'a' as u8) as usize] {
                return false;
            }

            t.push(c);
            exists[(c as u8 - 'a' as u8) as usize] = true;
        }
    }

    true
}
