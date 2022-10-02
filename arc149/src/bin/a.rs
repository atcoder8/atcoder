use proconio::input;

fn main() {
    if let Some((max_digits_num, d)) = solve() {
        let ans = std::char::from_digit(d as u32, 10)
            .unwrap()
            .to_string()
            .repeat(max_digits_num);
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<(usize, usize)> {
    input! {
        (n, m): (usize, usize),
    }

    if n <= 9 && 10_usize.pow(n as u32) <= m {
        return None;
    }

    let (max_digits_num, d) = (1..10)
        .map(|d| {
            let mut r = 0;
            let mut max_digits_num = 0;

            for digits_num in 1..=n {
                r = (10 * r + d) % m;

                if r == 0 {
                    max_digits_num = digits_num;
                }
            }

            (max_digits_num, d)
        })
        .max()
        .unwrap();

    if max_digits_num != 0 {
        Some((max_digits_num, d))
    } else {
        None
    }
}
