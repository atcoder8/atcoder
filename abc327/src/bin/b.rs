use proconio::input;

fn main() {
    input! {
        b: usize,
    }

    match solve(b) {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve(b: usize) -> Option<usize> {
    for a in 1_usize.. {
        match a.checked_pow(a as u32) {
            Some(pow) => {
                if pow == b {
                    return Some(a);
                } else if pow > b {
                    return None;
                }
            }
            None => return None,
        }
    }

    unreachable!()
}
