use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        s: Chars,
    }

    let ans = (0..n)
        .map(|replace_num| {
            let mut t = s.clone();
            t.rotate_left(replace_num);

            a * replace_num + b * (0..(n / 2)).filter(|&i| t[i] != t[n - 1 - i]).count()
        })
        .min()
        .unwrap();
    println!("{}", ans);
}
