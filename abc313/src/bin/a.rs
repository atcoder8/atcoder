use proconio::input;

fn main() {
    input! {
        n: usize,
        pp: [usize; n],
    }

    let max = *pp.iter().max().unwrap();
    let ans = if pp[0] == max && pp.iter().filter(|&&p| p == max).count() == 1 {
        0
    } else {
        max + 1 - pp[0]
    };
    println!("{}", ans);
}
