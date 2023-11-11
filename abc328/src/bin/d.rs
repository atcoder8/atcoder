use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut stack = vec![];
    for &c in &s {
        if c == 'C'
            && stack.len() >= 2
            && stack[stack.len() - 2] == 'A'
            && stack[stack.len() - 1] == 'B'
        {
            stack.pop();
            stack.pop();
        } else {
            stack.push(c);
        }
    }

    let ans: String = stack.iter().collect();
    println!("{}", ans);
}
