use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let calc_sum = |di: u32| {
        let mut sum = 0_usize;
        let mut counts = [0; 2];
        for &a in &aa {
            let parity = a >> di & 1;
            sum += counts[parity ^ 1];
            if parity == 1 {
                counts.swap(0, 1);
            }
            counts[parity] += 1;
        }

        sum << di
    };

    let ans = (0..30).map(calc_sum).sum::<usize>();
    println!("{}", ans);
}
