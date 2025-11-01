use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        s: Chars,
    }

    let mut acc_a = vec![0_usize; n + 1];
    let mut acc_b = vec![0_usize; n + 1];
    for i in 0..n {
        acc_a[i + 1] = acc_a[i] + (s[i] == 'a') as usize;
        acc_b[i + 1] = acc_b[i] + (s[i] == 'b') as usize;
    }

    let mut num_pairs = 0_usize;
    for i in 0..n {
        let border_a = acc_a[i + 1..].partition_point(|&cnt| cnt < a + acc_a[i]);
        let border_b = acc_b[i + 1..].partition_point(|&cnt| cnt < b + acc_b[i]);
        num_pairs += border_b.saturating_sub(border_a);
    }
    println!("{num_pairs}");
}
