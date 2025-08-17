use proconio::input;

const DICT: [(&str, &str); 3] = [("red", "SSS"), ("blue", "FFF"), ("green", "MMM")];

fn main() {
    input! {
        s: String,
    }

    let elem = DICT.iter().find(|elem| elem.0 == &s);
    match elem {
        Some(elem) => println!("{}", elem.1),
        None => println!("Unknown"),
    }
}
