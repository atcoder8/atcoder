use itertools::{enumerate, izip, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let mut to_small = vec![];
    let mut to_large = vec![];
    for (i, (&c1, &c2)) in enumerate(izip!(&s, &t)) {
        if c1 > c2 {
            to_small.push(i);
        } else if c1 < c2 {
            to_large.push(i);
        }
    }

    let mut x: Vec<String> = vec![];
    for &i in &to_small {
        s[i] = t[i];
        x.push(s.iter().collect());
    }
    for &i in to_large.iter().rev() {
        s[i] = t[i];
        x.push(s.iter().collect());
    }

    let ans = x.iter().join("\n");
    println!("{}\n{}", x.len(), ans);
}
