use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        (_n, r, c): (usize, i64, i64),
        s: String,
    }

    let mut output = String::new();

    let mut offset = (0, 0);
    let mut coordinates = BTreeSet::<(i64, i64)>::from([(0, 0)]);
    for ch in s.chars() {
        let relative = decode_direction(ch);
        offset = (offset.0 + relative.0, offset.1 + relative.1);
        coordinates.insert((-offset.0, -offset.1));

        let exists = coordinates.contains(&(r - offset.0, c - offset.1));
        output.push(if exists { '1' } else { '0' });
    }

    println!("{}", output);
}

fn decode_direction(c: char) -> (i64, i64) {
    match c {
        'N' => (-1, 0),
        'W' => (0, -1),
        'S' => (1, 0),
        'E' => (0, 1),
        _ => panic!(),
    }
}
