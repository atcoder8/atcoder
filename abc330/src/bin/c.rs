use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        d: usize,
    }

    let mut ans = d;
    for x in 0..=d.sqrt() {
        let y = (d - x.pow(2)).sqrt();
        ans = ans.min(d.abs_diff(x.pow(2) + y.pow(2)));
        ans = ans.min(d.abs_diff(x.pow(2) + (y + 1).pow(2)));
    }

    println!("{}", ans);
}
