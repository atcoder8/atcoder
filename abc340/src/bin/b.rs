use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut a = vec![];
    for _ in 0..q {
        input! {
            qt: usize,
        }

        if qt == 1 {
            input! {
                x: usize,
            }

            a.push(x);
        } else {
            input! {
                k: usize,
            }

            let ans = a[a.len() - k];
            println!("{}", ans);
        }
    }
}
