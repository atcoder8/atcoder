use itertools::{enumerate, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut imos = vec![0_usize; n];
    for (i, &d) in enumerate(&s) {
        imos[i] += d.to_digit(10).unwrap() as usize * (i + 1);
    }
    for i in 0..n - 1 {
        imos[i + 1] += imos[i];
    }

    let mut carry = 0_usize;
    let mut digits = vec![];
    for &d in imos.iter().rev() {
        let d = d + carry;
        digits.push(d % 10);
        carry = d / 10;
    }
    while carry != 0 {
        digits.push(carry % 10);
        carry /= 10;
    }
    digits.reverse();

    println!("{}", digits.iter().join(""));
}
