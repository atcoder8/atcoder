use proconio::{input, marker::Chars};

fn main() {
    input! {
        ss: [Chars; 8],
    }

    for i in 0..8 {
        for j in 0..8 {
            if ss[i][j] == '*' {
                println!("{}{}", (b'a' + j as u8) as char, 8 - i);
                std::process::exit(0);
            }
        }
    }
}
