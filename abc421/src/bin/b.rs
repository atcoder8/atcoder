use proconio::input;

fn main() {
    input! {
        (x, y): (usize, usize),
    }

    let mut aa = vec![x, y];
    for i in 2..10 {
        aa.push(rev(aa[i - 2] + aa[i - 1]));
    }
    println!("{}", aa[9]);
}

fn rev(a: usize) -> usize {
    a.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}
