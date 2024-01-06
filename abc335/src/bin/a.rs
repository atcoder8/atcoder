use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    *s.last_mut().unwrap() = '4';
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
