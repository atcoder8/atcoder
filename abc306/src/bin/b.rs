use proconio::input;

fn main() {
    input! {
        aa: [usize; 64],
    }

    let ans: usize = aa.iter().enumerate().map(|(i, &a)| a << i).sum();
    println!("{}", ans);
}
