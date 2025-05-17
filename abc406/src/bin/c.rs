use proconio::input;

fn main() {
    println!("{}", solve());
}

fn solve() -> usize {
    input! {
        n: usize,
        pp: [usize; n],
    }

    let Some(mut left) = (0..n - 1).find(|&i| pp[i] < pp[i + 1]) else {
        return 0;
    };
    let Some(mut max_pos) = (left + 1..n - 1).find(|&i| pp[i - 1] < pp[i] && pp[i] > pp[i + 1])
    else {
        return 0;
    };

    let mut ans = 0_usize;
    loop {
        let Some(min_pos) = (max_pos + 1..n - 1).find(|&i| pp[i - 1] > pp[i] && pp[i] < pp[i + 1])
        else {
            break;
        };
        let right = (min_pos..n - 1)
            .find(|&i| pp[i] > pp[i + 1])
            .unwrap_or(n - 1);

        ans += (max_pos - left) * (right - min_pos);

        left = min_pos;
        max_pos = right;
    }

    ans
}
