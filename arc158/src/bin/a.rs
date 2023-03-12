use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            xx: [usize; 3],
        }

        if let Some(ans) = solve(xx) {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}

fn solve(xx: Vec<usize>) -> Option<usize> {
    let mut xx = xx;
    xx.sort_unstable();
    let min_x = *xx.iter().min().unwrap();
    xx.iter_mut().for_each(|x| *x -= min_x);
    let sum_x: usize = xx.iter().sum();

    if xx[0] % 2 != xx[1] % 2 || xx[0] % 2 != xx[2] % 2 || sum_x % 6 != 0 {
        return None;
    }

    let mut ans = 0;

    while xx[1] > xx[0] + 2 {
        let add_num = (xx[1] - xx[0]) / 4;

        xx[0] += 4 * add_num;

        let other_add = 2 * add_num;

        if xx[2] - xx[1] >= other_add {
            xx[1] += other_add;
        } else {
            let rem = other_add - (xx[2] - xx[1]);
            let half = rem / 4 * 2;

            xx[1] = xx[2] + half;
            xx[2] += rem - half;
        }

        ans += add_num;
    }

    ans += (2 * xx[2] - (xx[0] + xx[1])) / 6;

    return Some(ans);
}
