use itertools::{join, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [Usize1; m],
    }

    let mut ans = vec![];

    let mut flags = vec![false; n];
    for &a in &aa {
        flags[a] = true;
    }

    let mut prev = 0;

    for i in 0..n {
        if !flags[i] {
            ans.append(&mut (prev..=i).rev().collect_vec());
            prev = i + 1;
        }
    }

    ans.iter_mut().for_each(|elem| *elem += 1);
    println!("{}", join(ans, " "));
}
