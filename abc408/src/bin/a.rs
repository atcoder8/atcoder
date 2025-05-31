use proconio::input;

fn main() {
    input! {
        (n, s): (usize, usize),
        tt: [usize; n],
    }

    let mut limit = s;
    let ans = tt.iter().fold(true, |acc, &t| {
        let next_acc = acc && t <= limit;
        limit = t + s;
        next_acc
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
