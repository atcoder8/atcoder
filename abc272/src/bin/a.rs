use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    println!("{}", aa.iter().sum::<usize>());
}
