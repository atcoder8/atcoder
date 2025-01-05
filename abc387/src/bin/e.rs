use proconio::input;

fn main() {
    match solve() {
        Some(ans) => println!("{}", ans),
        None => println!("-1"),
    }
}

fn solve() -> Option<String> {
    input! {
        n: String,
    }

    let num_digits = n.len();

    if n.len() <= 5 {
        let n = n.parse::<usize>().unwrap();
        return (n..2 * n).find(|&a| is_good(a)).map(|a| a.to_string());
    }

    let digits = str_to_digits(&n);
    let good_number = match digits[0] {
        1 => {
            if digits[1] <= 6 {
                build_good_number("17", num_digits)
            } else {
                build_good_number("26", num_digits)
            }
        }
        2..=7 => {
            let prefix = format!("{}{}", digits[0] + 1, 7 - digits[0]);
            build_good_number(&prefix, num_digits)
        }
        8..=9 => build_good_number("116", num_digits + 1),
        _ => panic!(),
    };
    Some(good_number)
}

fn str_to_digits(n: &str) -> Vec<usize> {
    n.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn calc_digit_sum(n: usize) -> usize {
    str_to_digits(&n.to_string()).iter().sum()
}

fn is_good(n: usize) -> bool {
    n % calc_digit_sum(n) == 0 && (n + 1) % calc_digit_sum(n + 1) == 0
}

fn build_good_number(prefix: &str, whole_length: usize) -> String {
    let zero_length = whole_length - prefix.len();
    format!("{}{}", prefix, "0".repeat(zero_length))
}
