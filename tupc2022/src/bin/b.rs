use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        mut ab: [(Usize1, usize); m],
    }

    ab.push((n, 0));

    let mut ans = 0_usize;
    let mut prev_day = 0_usize;
    let mut prev_snow_volume = 0_usize;

    for &(day, snowfall) in &ab {
        let diff_day = day - prev_day;
        ans += diff_day.min(prev_snow_volume.saturating_sub(k));

        prev_day = day;
        prev_snow_volume = prev_snow_volume.saturating_sub(diff_day) + snowfall;
    }

    println!("{}", ans);
}
