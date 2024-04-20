use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        tt: [Usize1; q],
    }

    let mut flags = vec![true; n];
    for &t in &tt {
        flags[t] = !flags[t];
    }

    let ans = flags.iter().filter(|&&f| f).count();
    println!("{}", ans);
}
