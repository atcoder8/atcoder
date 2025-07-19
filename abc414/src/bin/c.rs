use proconio::input;

fn main() {
    input! {
        a: usize,
        n: usize,
    }

    let num_digits_n = n.to_string().len();

    let calc_sum = |odd: bool| {
        let mut sum = 0_usize;

        for half in 1.. {
            let digits_10 = double(half, 10, odd);
            if digits_10.len() > num_digits_n {
                break;
            }

            let value = digits_to_usize(&digits_10, 10);
            if value > n {
                break;
            }

            let digits_a = retrieve_digits(value, a);
            let is_palindrome =
                (0..digits_a.len() / 2).all(|i| digits_a[i] == digits_a[digits_a.len() - 1 - i]);

            if is_palindrome {
                sum += value;
            }
        }

        sum
    };

    let ans = calc_sum(true) + calc_sum(false);
    println!("{}", ans);
}

fn retrieve_digits(n: usize, radix: usize) -> Vec<usize> {
    let mut digits = vec![];
    let mut t = n;
    while t != 0 {
        digits.push(t % radix);
        t /= radix;
    }
    digits.reverse();

    digits
}

fn double(k: usize, radix: usize, odd: bool) -> Vec<usize> {
    let mut digits = retrieve_digits(k, radix);
    let mut rev_digits = digits[..digits.len() - (odd as usize)].to_vec();
    rev_digits.reverse();
    digits.append(&mut rev_digits);

    digits
}

fn digits_to_usize(digits: &[usize], radix: usize) -> usize {
    let mut value = 0_usize;
    for d in digits {
        value = value * radix + d;
    }

    value
}
