use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    println!("{}", solve(n));
}

fn solve(mut n: u128) -> String {
    if n == 1 {
        return 0.to_string();
    }

    n -= 1;

    for digit_num in 1.. {
        let effective_digit_num = (digit_num + 1) / 2;

        let comb_num = 9 * 10_u128.pow(effective_digit_num - 1);
        if n <= comb_num {
            let mut s = (10_u128.pow(effective_digit_num - 1) + n - 1).to_string();
            let half = s[..s.len() - digit_num as usize % 2].to_string();
            s.extend(half.chars().rev());

            return s;
        }

        n -= comb_num;
    }

    unreachable!();
}
