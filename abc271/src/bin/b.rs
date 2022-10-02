use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut aaa = vec![];
    for _ in 0..n {
        input! {
            l: usize,
            aa: [usize; l],
        }

        aaa.push(aa);
    }

    input! {
        st: [(Usize1, Usize1); q],
    }

    for (s, t) in st {
        println!("{}", aaa[s][t]);
    }
}
