use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut aa: [usize; n],
        bb: [usize; m],
    }

    for &b in &bb {
        if let Some(pos) = aa.iter().position(|&a| a == b) {
            aa.remove(pos);
        }
    }

    if !aa.is_empty() {
        println!("{}", aa.iter().join(" "));
    }
}
