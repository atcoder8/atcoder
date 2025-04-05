use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u32,
    }

    match solve(n, m) {
        Some(ans) => println!("{}", ans),
        None => println!("inf"),
    }
}

fn solve(n: usize, m: u32) -> Option<usize> {
    let mut sum = 0_usize;
    for i in 0..=m {
        sum += n.pow(i);

        if sum > 10_usize.pow(9) {
            return None;
        }
    }

    Some(sum)
}
