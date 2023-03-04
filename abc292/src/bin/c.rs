use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let ans: usize = (1..n)
        .map(|ab| {
            let cd = n - ab;

            calc_divisor_num(ab) * calc_divisor_num(cd)
        })
        .sum();
    println!("{}", ans);
}

pub fn calc_divisor_num(n: usize) -> usize {
    let mut divisor_num = 0;

    for i in (1..).take_while(|&i| i * i <= n) {
        if n % i == 0 {
            divisor_num += 1;

            if n / i != i {
                divisor_num += 1;
            }
        }
    }

    divisor_num
}
