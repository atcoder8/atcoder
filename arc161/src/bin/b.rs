use proconio::input;

fn main() {
    input! {
        t: usize,
        nn: [usize; t],
    }

    for &n in &nn {
        if let Some(ans) = solve(n) {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}

fn lsb(n: usize) -> usize {
    1_usize << n.trailing_zeros()
}

fn reduce_bit(n: usize) -> usize {
    let mut ans = n;
    let one_num = n.count_ones();
    for _ in 0..(one_num - 3) {
        ans -= lsb(ans);
    }

    ans
}

fn solve(n: usize) -> Option<usize> {
    if n < 0b111 {
        return None;
    }

    let pop_num = n.count_ones();
    let lsb_n = lsb(n);

    let inc_bit = match pop_num {
        1 => n - 1,
        2 => {
            if lsb_n >= 0b100 {
                n - 1
            } else {
                n - lsb_n - 1
            }
        }
        _ => n,
    };

    Some(reduce_bit(inc_bit))
}
