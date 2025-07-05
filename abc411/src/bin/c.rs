use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [Usize1; q],
    }

    let mut cnt = 0_usize;
    let mut colors = vec![false; n];
    for &a in &aa {
        let left_color = a > 0 && colors[a - 1];
        let right_color = a + 1 < n && colors[a + 1];

        if colors[a] {
            cnt -= 1;
            cnt += left_color as usize;
            cnt += right_color as usize;
        } else {
            cnt += 1;
            cnt -= left_color as usize;
            cnt -= right_color as usize;
        }

        println!("{}", cnt);

        colors[a] = !colors[a];
    }
}
