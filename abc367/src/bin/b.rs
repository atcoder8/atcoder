use proconio::input;

fn main() {
    input! {
        mut x: String,
    }

    while x.ends_with('0') {
        x.pop();
    }
    if x.ends_with('.') {
        x.pop();
    }
    println!("{}", x);
}
