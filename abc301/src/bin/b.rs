use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    for i in 0..(n - 1) {
        if aa[i] < aa[i + 1] {
            for v in aa[i]..aa[i + 1] {
                print!("{} ", v);
            }
        } else {
            for v in ((aa[i + 1] + 1)..=aa[i]).rev() {
                print!("{} ", v);
            }
        }
    }
    println!("{}", aa[n - 1]);
}
