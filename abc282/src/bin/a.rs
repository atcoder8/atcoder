use proconio::input;

fn main() {
    input! {
        k: u8,
    }

    let chars: String = (0..k).map(|i| (b'A' + i) as char).collect();
    println!("{}", chars);
}
