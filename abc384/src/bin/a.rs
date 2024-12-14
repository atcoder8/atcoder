use proconio::input;

fn main() {
    input! {
        (_n, c1, c2): (usize, char, char),
        s: String,
    }

    let ans = s
        .chars()
        .map(|c| if c == c1 { c1 } else { c2 })
        .collect::<String>();
    println!("{}", ans);
}
