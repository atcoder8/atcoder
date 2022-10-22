use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
    }

    let mut generations = vec![0_usize];

    for &a in &aa {
        generations.push(generations[a] + 1);
        generations.push(generations[a] + 1);
    }

    for &generation in &generations {
        println!("{}", generation);
    }
}
