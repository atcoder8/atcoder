use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (s, t): (Chars, Chars),
    }

    for k in 0..26 {
        let rotated_s: Vec<char> = s.iter().map(|x| {
            std::char::from_u32((*x as u32 - 'a' as u32 + k) % 26 + 'a' as u32).unwrap()
        }).collect();
        if rotated_s == t {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
