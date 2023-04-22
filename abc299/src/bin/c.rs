use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut max_level = 0;
    let mut level = 0;
    let mut pre_hyphen = false;

    for &c in &s {
        if c == '-' {
            max_level = max_level.max(level);
            level = 0;
            pre_hyphen = true;
        } else {
            level += 1;
        }
    }

    if pre_hyphen {
        max_level = max_level.max(level);
    }

    if max_level == 0 {
        println!("-1");
    } else {
        println!("{}", max_level);
    }
}
