use proconio::input;

fn main() {
    input! {
        (n, t): (usize, usize),
        cc: [usize; n],
        rr: [usize; n],
    }

    let winner = (0..n).filter(|&i| cc[i] == t).max_by_key(|&i| rr[i]);
    let winner = if let Some(winner) = winner {
        winner
    } else {
        (0..n).filter(|&i| cc[i] == cc[0]).max_by_key(|&i| rr[i]).unwrap()
    };

    println!("{}", winner + 1);
}
