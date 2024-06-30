use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let pos_r = s.chars().position(|c| c == 'R');
    let pos_m = s.chars().position(|c| c == 'M');

    println!("{}", if pos_r < pos_m { "Yes" } else { "No" });
}
