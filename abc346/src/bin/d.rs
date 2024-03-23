use proconio::{input, marker::Chars};

const INF: usize = 10_usize.pow(16);

fn main() {
    input! {
        n: usize,
        s: Chars,
        cc: [usize; n],
    }

    let mut bad = [0, cc[0]];
    if s[0] == '1' {
        bad.swap(0, 1);
    }
    let mut good = [INF, INF];

    for i in 1..n {
        let mut costs = [0, cc[i]];
        if s[i] == '1' {
            costs.swap(0, 1);
        }

        (bad, good) = (
            [bad[1] + costs[0], bad[0] + costs[1]],
            [
                bad[0].min(good[1]) + costs[0],
                bad[1].min(good[0]) + costs[1],
            ],
        );
    }

    println!("{}", good[0].min(good[1]));
}
