use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let upper_num = s.chars().filter(|&c| c.is_ascii_uppercase()).count();
    let ans = if upper_num > s.len() - upper_num {
        s.to_uppercase()
    } else {
        s.to_lowercase()
    };

    println!("{}", ans);
}
