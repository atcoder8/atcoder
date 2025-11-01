// unfinished

use itertools::{enumerate, izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve().iter().join(" ")).join("\n");
    println!("{output}");
}

fn solve() -> Vec<i64> {
    input! {
        n: usize,
        s: Chars,
    }

    let mut offset_left = vec![0_usize; n];
    let mut offset_right = vec![0_usize; n];

    let mut prev = ('.', 0_usize);
    for (i, &ch) in enumerate(&s) {
        offset_left[i] = offset_left[i].max(prev.1 * (prev.0 == 'R') as usize);
        offset_right[i] = offset_right[i].max(prev.1 * (prev.0 == 'L') as usize);

        if ch == prev.0 {
            prev.1 += 1;
        } else {
            prev = (ch, 1);
        }
    }
    offset_left[n - 1] = offset_left[n - 1].max(prev.1 * (prev.0 == 'R') as usize);
    offset_right[n - 1] = offset_right[n - 1].max(prev.1 * (prev.0 == 'L') as usize);

    let mut prev = ('.', 0_usize);
    for (i, &ch) in enumerate(&s).rev() {
        offset_left[i + 1] = offset_left[i + 1].max(prev.1 * (prev.0 == 'L') as usize);
        offset_right[i + 1] = offset_right[i + 1].max(prev.1 * (prev.0 == 'R') as usize);

        if ch == prev.0 {
            prev.1 += 1;
        } else {
            prev = (ch, 1);
        }
    }
    offset_left[0] = offset_left[0].max(prev.1 * (prev.0 == 'L') as usize);
    offset_right[0] = offset_right[0].max(prev.1 * (prev.0 == 'R') as usize);

    let mut imos = vec![0_i64; n + 1];
    for (&left, &right) in izip!(&offset_left, &offset_right) {
        imos[left] += 1;
        imos[n - right] -= 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    imos.pop();
    imos
}
