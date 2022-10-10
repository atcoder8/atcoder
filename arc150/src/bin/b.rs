use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        println!("{}", solve());
    }
}

fn solve() -> usize {
    input! {
        (a, b): (usize, usize),
    }

    if a == 1 {
        return 0;
    }

    if b <= a {
        return a - b;
    }

    let mut ans = b - a;

    for k in (1..).take_while(|&k| k * k <= b - 1) {
        let x = ((b - 1) / k + 1).saturating_sub(a);
        let y = k * (a + x) - b;

        ans = ans.min(x + y);
    }

    for q in (1..).take_while(|&q| q * q <= b - 1) {
        let k = (b - 1) / (q + 1) + 1;

        let x = ((b - 1) / k + 1).saturating_sub(a);
        let y = k * (a + x) - b;

        ans = ans.min(x + y);
    }

    ans
}
