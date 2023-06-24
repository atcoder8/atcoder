use itertools::join;
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; 7 * n],
    }

    let mut ans: Vec<usize> = vec![];
    for i in (0..(7 * n)).step_by(7) {
        ans.push(aa[i..(i + 7)].iter().sum());
    }

    println!("{}", join(ans, " "));
}
