use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = vec![];
    let mut x = n;

    while x > 0 {
        ans.push(x);
        x = (x - 1) & n;
    }
    ans.push(0);

    for &e in ans.iter().rev() {
        println!("{}", e);
    }
}
