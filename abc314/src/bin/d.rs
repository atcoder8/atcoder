use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
        txc: [(usize, usize, char); q],
    }

    let mut history = vec![0; n];
    let mut case = Case::None;
    for (i, &(t, x, c)) in txc.iter().enumerate() {
        if t == 1 {
            s[x - 1] = c;
            history[x - 1] = i;
        } else if t == 2 {
            case = Case::Lower(i);
        } else {
            case = Case::Upper(i);
        }
    }

    for (i, &c) in s.iter().enumerate() {
        let c = match case {
            Case::None => c,
            Case::Lower(time) => {
                if history[i] > time {
                    c
                } else {
                    c.to_ascii_lowercase()
                }
            }
            Case::Upper(time) => {
                if history[i] > time {
                    c
                } else {
                    c.to_ascii_uppercase()
                }
            }
        };
        print!("{}", c);
    }
    println!();
}

enum Case {
    None,
    Lower(usize),
    Upper(usize),
}
