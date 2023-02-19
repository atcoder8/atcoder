use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let mut flags = vec![false; k];
    for &a in &aa {
        if a >= k {
            continue;
        }

        flags[a] = true;
    }

    let mex = (0..k).take_while(|&i| flags[i]).count();
    println!("{}", mex);
}
