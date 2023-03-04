use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut cards = vec![0_usize; n];

    for _ in 0..q {
        input! {
            (query_type, x): (usize, Usize1),
        }

        if query_type == 1 {
            cards[x] += 1;
        } else if query_type == 2 {
            cards[x] = 2;
        } else {
            println!("{}", if cards[x] == 2 { "Yes" } else { "No" });
        }
    }
}
