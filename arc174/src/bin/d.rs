use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        nn: [usize; t],
    }

    println!("{}", nn.iter().map(|&n| solve(n)).join("\n"));
}

fn solve(n: usize) -> usize {
    let mut ans = 1;
    for i in 1..=9 {
        let pow = 10_usize.pow(i);

        if n >= pow * (pow - 2) {
            ans += 1;
        }

        ans += (pow * (pow + 1)).min(n + 1).saturating_sub(pow * (pow - 1));
    }

    ans
}

#[cfg(test)]
mod tests {
    use num_integer::Roots;

    fn is_ok(x: usize) -> bool {
        let y = x.sqrt();
        let str_x = x.to_string();
        let str_y = y.to_string();

        str_x[..str_y.len()] == str_y
    }

    #[test]
    fn test() {
        let mut start = 1;
        let mut len = 1;
        for x in 2..=3 * 10_usize.pow(8) {
            if is_ok(x) {
                if x == start + len {
                    len += 1;
                } else {
                    start = x;
                    len = 1;
                }
            } else {
                if len != 0 {
                    println!("[{}, {}] (Length: {})", start, start + len - 1, len);
                }

                len = 0;
            }
        }

        if len != 0 {
            println!("[{}, {}?] (Length: {}?)", start, start + len - 1, len);
        }
    }
}
