use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [usize; n],
    }

    let mut bb = aa.clone();
    bb.sort_unstable();

    let mut rem = k;
    let mut round_num = 0;

    for (i, &b) in bb.iter().enumerate() {
        let quo = rem / (n - i);

        if quo >= b - round_num {
            rem -= (b - round_num) * (n - i);
            round_num = b;

            if rem == 0 {
                break;
            }
        } else {
            rem -= (n - i) * quo;
            round_num += quo;

            break;
        }
    }

    let mut extra_eat = 0;

    let ans = aa.iter().map(|&a| {
        if a <= round_num {
            0
        } else if extra_eat < rem {
            extra_eat += 1;
            a - round_num - 1
        } else {
            a - round_num
        }
    });

    println!("{}", itertools::join(ans, " "));
}
