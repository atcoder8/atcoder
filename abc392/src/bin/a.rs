use proconio::input;

fn main() {
    input! {
        mut aa: [usize; 3],
    }

    aa.sort_unstable();

    println!("{}", if aa[0] * aa[1] == aa[2] { "Yes" } else { "No" });
}
