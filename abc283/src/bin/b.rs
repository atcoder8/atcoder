use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
        q: usize,
    }

    for _ in 0..q {
        input! {
            query_type: usize,
        }

        if query_type == 1 {
            input! {
                (k, x): (Usize1, usize),
            }

            aa[k] = x;
        } else {
            input! {
                k: Usize1,
            }

            println!("{}", aa[k]);
        }
    }
}
