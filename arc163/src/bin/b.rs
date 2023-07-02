use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [usize; n],
    }

    let mut sub_aa = aa[2..].to_owned();
    sub_aa.sort_unstable();

    let mut ans = 2e18 as usize;

    for left in 0..sub_aa.len() {
        let right = left + m - 1;

        if right >= sub_aa.len() {
            break;
        }

        let min = sub_aa[left];
        let max = sub_aa[right];

        let cost = aa[0].saturating_sub(min) + max.saturating_sub(aa[1]);
        ans = ans.min(cost);
    }

    println!("{}", ans);
}
