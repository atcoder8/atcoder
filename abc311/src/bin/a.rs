use proconio::{input, marker::Bytes};

fn main() {
    input! {
        _n: usize,
        s: Bytes,
    }

    let mut bit = 0;
    for (i, &c) in s.iter().enumerate() {
        bit |= 1 << (c - b'A');

        if bit == 0b111 {
            println!("{}", i + 1);
            break;
        }
    }
}
