use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans = (1_usize..)
        .take_while(|&i| i.pow(3) <= n)
        .filter(|&i| {
            let s = i.pow(3).to_string().chars().collect_vec();
            let mut rev_s = s.clone();
            rev_s.reverse();

            s == rev_s
        })
        .max()
        .unwrap()
        .pow(3);
    println!("{}", ans);
}
