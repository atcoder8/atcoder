use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let max_a = *aa.iter().max().unwrap();

    let mut acc = vec![0_usize; max_a + 1];
    for &a in &aa {
        acc[a] += 1;
    }
    for i in 0..max_a {
        acc[i + 1] += acc[i];
    }

    let mut ans = 0;
    for i in 1..=max_a {
        let num = acc[i] - acc[i - 1];
        ans += num * num.saturating_sub(1) / 2;

        for w in (1..).take_while(|&w| i * w <= max_a) {
            let left = (i * w).max(i + 1);
            let right = (i * (w + 1) - 1).min(max_a);

            ans += num * (acc[right] - acc[left - 1]) * w;
        }
    }

    println!("{}", ans);
}
