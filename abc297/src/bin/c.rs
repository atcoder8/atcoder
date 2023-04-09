use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        mut ss: [Chars; h],
    }

    for i in 0..h {
        for j in 0..(w - 1) {
            if ss[i][j] == 'T' && ss[i][j + 1] == 'T' {
                ss[i][j] = 'P';
                ss[i][j + 1] = 'C';
            }
        }
    }

    for s in &ss {
        for &c in s {
            print!("{}", c);
        }
        println!();
    }
}
