use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut t = 0;
    let mut a = 0;

    for &c in &s {
        if c == 'T' {
            t += 1;

            if t == (n + 1) / 2 {
                println!("T");
                std::process::exit(0);
            }
        } else {
            a += 1;

            if a == (n + 1) / 2 {
                println!("A");
                std::process::exit(0);
            }
        }
    }
}
