use proconio::{input, marker::Usize1};

const MAX: usize = 10_usize.pow(6);

fn main() {
    input! {
        q: usize,
    }

    let mut counts = vec![0; MAX];
    let mut num_types = 0;

    for _ in 0..q {
        input! {
            qt: usize,
        }

        match qt {
            1 => {
                input! {
                    x: Usize1,
                }

                counts[x] += 1;

                if counts[x] == 1 {
                    num_types += 1;
                }
            }
            2 => {
                input! {
                    x: Usize1,
                }

                counts[x] -= 1;

                if counts[x] == 0 {
                    num_types -= 1;
                }
            }
            3 => {
                println!("{}", num_types);
            }
            _ => panic!(),
        }
    }
}
