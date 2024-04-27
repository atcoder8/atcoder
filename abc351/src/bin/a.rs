use proconio::input;

fn main() {
    input! {
        aa: [usize; 9],
        bb: [usize; 8],
    }

    let sum_a = aa.iter().sum::<usize>();
    let sum_b = bb.iter().sum::<usize>();
    let ans = sum_a - sum_b + 1;
    println!("{}", ans);
}
