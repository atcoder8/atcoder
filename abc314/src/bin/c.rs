use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        (n, m): (usize, usize),
        s: Chars,
        cc: [Usize1; n],
    }

    let mut chars_by_color = vec![VecDeque::new(); m];
    for (&ch, &c) in s.iter().zip(&cc) {
        chars_by_color[c].push_back(ch);
    }
    for chars in &mut chars_by_color {
        let last = chars.pop_back().unwrap();
        chars.push_front(last);
    }

    for &c in &cc {
        print!("{}", chars_by_color[c].pop_front().unwrap());
    }
    println!();
}
