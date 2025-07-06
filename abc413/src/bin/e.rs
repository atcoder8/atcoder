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

    for exp in (1..=n).rev() {
        for left in (0..2_usize.pow(n)).step_by(2_usize.pow(exp)) {
            let section = &mut pp[left..left + 2_usize.pow(exp)];
            if section.iter().position_min().unwrap() >= section.len() / 2 {
                section.reverse();
            }
        }
    }

    pp
}
