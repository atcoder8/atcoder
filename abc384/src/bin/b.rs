use proconio::input;

fn main() {
    input! {
        (n, r): (usize, i64),
        da: [(u8, i64); n],
    }

    let mut rating = r;
    for &(d, a) in &da {
        let is_rated =
            d == 1 && (1600..2800).contains(&rating) || d == 2 && (1200..2400).contains(&rating);
        rating += if is_rated { a } else { 0 }
    }
    println!("{}", rating);
}
