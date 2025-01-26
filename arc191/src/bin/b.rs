use itertools::{izip, Itertools};
use proconio::{fastout, input};

// X<Nのとき、X mod N = Xである。従って、XOR(X,N)=Xであるが、このときN=0なので成り立たない。
// X>=Nのとき、Xの桁数がNの桁数を上回る場合は成り立たない。桁数が同じとき、Xでビットが立っている集合がNでビットが立っている桁の集合を含む場合は成り立つ。
// Nでビットが立っていない桁は0または1を自由に選べる。

#[fastout]
fn main() {
    input! {
        t: usize,
        nk: [(usize, usize); t],
    }

    for &(n, k) in &nk {
        match solve(n, k) {
            Some(ans) => println!("{}", ans),
            None => println!("-1"),
        }
    }
}

fn solve(n: usize, k: usize) -> Option<usize> {
    let n_digits = to_digits(n);
    let zero_positions = n_digits.iter().positions(|&digit| digit == 0).collect_vec();

    if k > (1 << zero_positions.len()) {
        return None;
    }

    // ゼロの部分にk-1を当てはめる
    let mut digits = n_digits;
    let k1_digits = to_digits(k - 1);
    let iter = zero_positions.iter().cloned();
    for (pos, &digit) in izip!(
        iter.skip(zero_positions.len() - k1_digits.len()),
        &k1_digits
    ) {
        digits[pos] = digit;
    }

    Some(digits.iter().fold(0_usize, |acc, &digit| 2 * acc + digit))
}

fn to_digits(n: usize) -> Vec<usize> {
    let mut digits = vec![];
    let mut t = n;
    while t != 0 {
        digits.push(t % 2);
        t /= 2;
    }
    digits.reverse();

    digits
}
