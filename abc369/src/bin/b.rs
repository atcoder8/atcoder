use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a_s: [(Usize1, char); n],
    }

    let mut cost = 0;
    let mut left = a_s
        .iter()
        .find(|&&(_, s)| s == 'L')
        .map(|&(a, _)| a)
        .unwrap_or(0);
    let mut right = a_s
        .iter()
        .find(|&&(_, s)| s == 'R')
        .map(|&(a, _)| a)
        .unwrap_or(0);
    for &(a, s) in &a_s {
        match s {
            'L' => {
                cost += left.abs_diff(a);
                left = a;
            }
            'R' => {
                cost += right.abs_diff(a);
                right = a;
            }
            _ => panic!(),
        }
    }

    println!("{}", cost);
}
