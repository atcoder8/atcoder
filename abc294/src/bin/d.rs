use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
    }

    let mut used = vec![false; n];
    let mut min_wait_idx = 0;

    for _ in 0..q {
        input! {
            event_type: usize,
        }

        match event_type {
            2 => {
                input! {
                    x: Usize1,
                }

                used[x] = true;
            }
            3 => {
                while used[min_wait_idx] {
                    min_wait_idx += 1;
                }

                println!("{}", min_wait_idx + 1);
            }
            _ => {}
        }
    }
}
