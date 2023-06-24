use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut ans = vec![];
    let mut level = 0;
    for &c in &s {
        if c == '(' {
            level += 1;
            ans.push('(');
        } else if c == ')' {
            if level == 0 {
                ans.push(')');
            } else {
                while let Some(elem) = ans.pop() {
                    if elem == '(' {
                        break;
                    }
                }
                level -= 1;
            }
        } else {
            ans.push(c);
        }
    }

    let ans: String = ans.iter().collect();
    println!("{}", ans);
}
