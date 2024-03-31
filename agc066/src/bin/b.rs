use proconio::input;

fn main() {
    input! {}
}

#[cfg(test)]
mod tests {
    // use super::*;

    // use std::str::FromStr;

    use num::{BigUint, Zero};
    // use rand::{thread_rng, Rng};

    // fn f(x: usize) -> usize {
    //     let mut sum = 0;
    //     let mut t = x;
    //     while t != 0 {
    //         sum += t % 10;
    //         t /= 10;
    //     }

    //     sum
    // }

    fn calc_sum_digit(x: &BigUint) -> usize {
        x.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .sum::<usize>()
    }

    fn count_match(x: &BigUint) -> usize {
        let mut cnt = 0;
        let mut x = x.clone();
        loop {
            let next_x = &x * 2_usize;

            if calc_sum_digit(&next_x) >= calc_sum_digit(&x) {
                break;
            }

            cnt += 1;
            x = next_x;
        }

        cnt
    }

    // fn count(shifted_x: usize) -> usize {
    //     let mut cnt = 0;
    //     let mut t = shifted_x;
    //     while t % 2 == 0 && f(t) < f(t / 2) {
    //         cnt += 1;
    //         t /= 2;
    //     }

    //     cnt
    // }

    // fn count_big(shifted_x: &BigUint) -> usize {
    //     let mut cnt = 0;
    //     let mut t = shifted_x.clone();
    //     while &t % 2_usize == BigUint::zero() && sum_digit(&t) < sum_digit(&(&t / 2_usize)) {
    //         cnt += 1;
    //         t /= 2_usize;
    //     }

    //     cnt
    // }

    // fn count_inc_len(x: &BigUint) -> u32 {
    //     let mut x = x.clone();
    //     let mut inc_len = 0;
    //     loop {
    //         let next_x = 5_usize * &x;

    //         if calc_sum_digit(&next_x) <= calc_sum_digit(&x) {
    //             break;
    //         }

    //         inc_len += 1;
    //         x = next_x;
    //     }

    //     inc_len
    // }

    #[test]
    fn test() {
        // let shifted_x = BigUint::from_str(&("3".to_string() + &"0".repeat(10000 - 1))).unwrap();
        // println!("{}", count_big(&shifted_x));
        // let mut max_count = 0;
        // for x in 1..=100000000 {
        //     let cnt = count_match(&BigUint::from_str(&x.to_string()).unwrap());
        //     if cnt > max_count {
        //         println!("shifted_x: {}, cnt: {}", x, cnt);
        //         max_count = cnt;
        //     }
        // }

        let mut chained = BigUint::zero();
        for exp in 1..=100 {
            chained = BigUint::new(vec![10]).pow(100) * &chained + BigUint::new(vec![5]).pow(exp);
        }

        println!("{}", count_match(&chained));
        println!("{}", chained.to_string().len());

        // let x = BigUint::new(vec![5]).pow(49);
        // println!("{}", count_inc_len(&x));

        // let base = BigUint::from_str("5").unwrap();
        // let mut rng = thread_rng();

        // let mut max_cnt = 0;
        // for _ in 0..10_usize.pow(8) {
        //     let x = (0..100).fold(BigUint::zero(), |acc, _| {
        //         10_usize * &acc + rng.gen_range(0_usize..10)
        //     });
        //     let cnt = count_inc_len(&x);
        //     if cnt > max_cnt {
        //         println!("{}: {}", x, cnt);
        //         max_cnt = cnt;
        //     }
        // }
        // let mut max_cnt = 0;
        // for x in 1_usize..=10_usize.pow(8) {
        //     let x = BigUint::from_str(&x.to_string()).unwrap();
        //     let cnt = count_inc_len(&x);

        //     if cnt > max_cnt {
        //         println!("{}: {}", x, cnt);
        //         max_cnt = cnt;
        //     }
        // }

        // let raised = base.pow(50);

        // let str_raised = (raised * (73_usize * 103_usize)).to_string();
        // let chain =
        //     str_raised.clone() + &"0".repeat(50) + &str_raised + &"0".repeat(50) + &str_raised;
        // let chain = BigUint::from_str(&chain).unwrap();
        // let mut max_cnt = 0;
        // // println!("{}", count_match(&chain));
        // for mul in 1_usize..=1000000 {
        //     let x = mul * &chain;
        //     let cnt = count_match(&x);
        //     if cnt > max_cnt {
        //         println!("{} {}", mul, cnt);
        //         max_cnt = cnt;
        //     }
        // }

        // let chain = BigUint::from_str(&(raised.to_string() + &"0".repeat(50) + &raised.to_string())).unwrap();
        // println!("{}", count_match(&chain));
        // let max = (1_usize..=1000000)
        //     .max_by_key(|mul| count_match(&(mul * &raised)))
        //     .unwrap();
        // println!("{}", max);
        // for i in 1_usize..=10000 {
        //     let x = i * &raised;
        //     println!("{}: {}", i, count_match(&x));
        // }

        // let x = BigUint::from_str("5").unwrap().pow(51);
        // println!("{}", count_match(x));
    }
}
