use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    println!("{}", aa.iter().step_by(2).sum::<usize>());
}
