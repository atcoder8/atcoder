use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    }

    let mut cnt = n;
    let mut right = 0;
    for left in 0..n - 1 {
        right = right.max(left + 1);
        let diff = aa[right] - aa[left];
        while right < n - 1 && aa[right + 1] - aa[right] == diff {
            right += 1;
        }

        cnt += right - left;
    }

    println!("{}", cnt);
}
