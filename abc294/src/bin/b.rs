use itertools::join;
use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [[usize; w]; h],
    }

    for aa in aaa {
        println!("{}", join(aa.into_iter().map(|a| to_char(a)), ""));
    }
}

fn to_char(a: usize) -> char {
    if a == 0 {
        '.'
    } else {
        (b'A' + (a - 1) as u8) as char
    }
}
