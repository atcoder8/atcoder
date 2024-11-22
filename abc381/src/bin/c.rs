use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let find_length = |pivot: usize| {
        let level = (1..=pivot.min(n - 1 - pivot))
            .take_while(|&i| s[pivot - i] == '1' && s[pivot + i] == '2')
            .count();
        2 * level + 1
    };
    let ans = (0..n)
        .filter(|&pivot| s[pivot] == '/')
        .map(find_length)
        .max()
        .unwrap();
    println!("{}", ans);
}
