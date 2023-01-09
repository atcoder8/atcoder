use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    if let Some((s, i)) = solve() {
        println!("{}\n{}", s, i);
    } else {
        println!("-1");
    }
}

fn solve() -> Option<(String, usize)> {
    input! {
        n: usize,
        t: Chars,
    }

    let a = t[..n].to_vec();
    let b = t[n..].iter().cloned().rev().collect_vec();

    let x = a.iter().chain(&b).cloned().collect_vec();
    let y = b.iter().chain(&a).cloned().collect_vec();

    let mut zx = z_algorithm(&x);
    zx.push(0);

    let mut zy = z_algorithm(&y);
    zy.push(0);

    for i in 0..=n {
        if zx[2 * n - i] == i && zy[n + i] == n - i {
            return Some((t[..i].iter().chain(&t[(n + i)..]).collect(), i));
        }
    }

    None
}

pub fn z_algorithm<T>(seq: &Vec<T>) -> Vec<usize>
where
    T: Eq,
{
    if seq.is_empty() {
        return vec![];
    }

    let n = seq.len();

    let mut z = vec![0; n];
    let mut j = 0;

    for i in 1..n {
        if j + z[j] > i {
            z[i] = z[i - j].min(j + z[j] - i);
        }

        let k = &mut z[i];

        while i + *k < n && seq[*k] == seq[i + *k] {
            *k += 1;
        }

        if j + z[j] < i + z[i] {
            j = i;
        }
    }

    z[0] = n;

    z
}
