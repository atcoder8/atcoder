use proconio::input;

const BRACKETS: [(char, char); 3] = [('(', ')'), ('[', ']'), ('<', '>')];

fn main() {
    input! {
        s: String,
    }

    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match stack.last().cloned() {
            Some(prev_c) if BRACKETS.contains(&(prev_c, c)) => {
                stack.pop();
            }
            _ => stack.push(c),
        }
    }

    println!("{}", if stack.is_empty() { "Yes" } else { "No" });
}
