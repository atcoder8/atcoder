use num::{BigUint, Zero, One};

fn main() {
    let (n, x, y): (usize, usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let mut red = vec![BigUint::zero(); n + 1];
    red[n] = BigUint::one();
    let mut blue = vec![BigUint::zero(); n + 1];

    for i in (2..(n + 1)).rev() {
        let r = red[i].clone();
        red[i - 1] += r.clone();
        blue[i] += r.clone() * x;

        let b = blue[i].clone();
        red[i - 1] += b.clone();
        blue[i - 1] += b.clone() * y;
    }

    println!("{}", blue[1]);
}
