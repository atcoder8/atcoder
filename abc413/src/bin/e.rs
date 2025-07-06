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
        mut pp: [usize; 1 << n],
    }

    for exp in (1..=n).rev() {
        for left in (0..1 << n).step_by(1 << exp) {
            let section = &mut pp[left..left + (1 << exp)];
            if section.iter().position_min().unwrap() >= section.len() / 2 {
                section.reverse();
            }
        }
    }

    pp
}
