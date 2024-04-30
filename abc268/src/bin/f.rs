use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: [String; n],
    }

    let mut score = 0;
    let mut xy = vec![];

    for s in &ss {
        let mut x_cnt = 0;
        let mut sum = 0;

        for c in s.chars() {
            if c == 'X' {
                x_cnt += 1;
            } else {
                let d = c.to_digit(10).unwrap() as usize;
                score += x_cnt * d;
                sum += d;
            }
        }

        xy.push((x_cnt, sum));
    }

    xy.sort_unstable_by(|(x1, y1), (x2, y2)| (x2 * y1).cmp(&(x1 * y2)));

    let mut x_cnt = 0;
    for &(x, y) in &xy {
        score += x_cnt * y;
        x_cnt += x;
    }

    println!("{}", score);
}
