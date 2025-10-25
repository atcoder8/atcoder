use proconio::input;

fn main() {
    input! {
        (n, m, c): (usize, usize, usize),
        mut aa: [usize; n],
    }

    aa.sort_unstable();

    let mut sum = 0_usize;
    let mut num_encounters = 0_usize;
    let mut left = 0_usize;
    let mut right = 0_usize;
    let mut prev_left_a = aa[n - 1];
    while left < n {
        while num_encounters < c {
            let next_right = (right + 1..n).find(|&i| aa[i] != aa[right]).unwrap_or(n);
            num_encounters += next_right - right;
            right = next_right % n;
        }

        let interval = aa[left] + m * (aa[left] <= prev_left_a) as usize - prev_left_a;
        sum += interval * num_encounters;

        prev_left_a = aa[left];

        let next_left = (left + 1..n).find(|&i| aa[i] != aa[left]).unwrap_or(n);
        num_encounters -= next_left - left;
        left = next_left;
    }

    println!("{sum}");
}
