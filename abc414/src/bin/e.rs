use proconio::input;

fn main() {
    input! {
        n: u128,
    }

    let num_combs = n * (n - 1) / 2 + n - sum_floor_div(n);
    println!("{}", num_combs % 998244353);
}

pub fn sum_floor_div(n: u128) -> u128 {
    let mut sum = 0;

    let mut denom = 1;
    while denom <= n / denom {
        sum += n / denom;
        denom += 1;
    }

    while denom <= n {
        let divided = n / denom;
        let next_denom = n / divided + 1;
        sum += divided * (next_denom - denom);
        denom = next_denom
    }

    sum
}
