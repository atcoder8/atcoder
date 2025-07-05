use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    println!("{}", (0..t).map(|_| solve().iter().join(" ")).join("\n"));
}

fn solve() -> Vec<usize> {
    input! {
        n: u32,
        mut pp: [usize; 2_usize.pow(n)],
    }

    let raised = 2_usize.pow(n);

    for exp in (1..=n).rev() {
        let step = 2_usize.pow(exp);
        for left in (0..raised).step_by(step) {
            let left_min = pp[left..left + step / 2].iter().min().unwrap();
            let right_min = pp[left + step / 2..left + step].iter().min().unwrap();
            if left_min > right_min {
                pp[left..left + step].reverse();
            }
        }
    }

    pp
}
