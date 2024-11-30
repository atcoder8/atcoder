use proconio::input;

fn main() {
    input! {
        (_n, d): (usize, usize),
        s: String,
    }

    let num_empty_boxes = s.chars().filter(|&c| c == '.').count();
    println!("{}", num_empty_boxes + d);
}
