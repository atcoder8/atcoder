use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    for s in ss.iter().rev() {
        println!("{}", s);
    }
}
