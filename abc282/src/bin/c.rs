use proconio::{input, marker::Chars};

fn main() {
    input! {
        (_n, s): (usize, Chars),
    }

    let mut t = s.clone();

    let mut quote = false;

    for (i, &c) in s.iter().enumerate() {
        if c == '"' {
            quote = !quote;
        } else if c == ',' && !quote {
            t[i] = '.';
        }
    }

    let t: String = t.iter().collect();
    println!("{}", t);
}
