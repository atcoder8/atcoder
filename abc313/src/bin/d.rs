use std::io::Write;

use itertools::join;

fn main() {
    let (n, k) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut ss = vec![0; k + 1];
    let mut total = 0;
    for i in 0..(k + 1) {
        let t = read(&(0..k).map(|j| (i + j + 1) % (k + 1)).collect());

        ss[i] = t;
        total ^= t;
    }

    let mut aa = vec![0; n];
    for i in 0..(k + 1) {
        aa[i] = total ^ ss[i];
    }
    let sum = aa[..(k - 1)].iter().fold(0, |acc, &x| acc ^ x);
    for i in (k + 1)..n {
        let t = read(&(0..(k - 1)).chain(vec![i]).collect());
        aa[i] = sum ^ t;
    }

    println!("! {}", join(aa, " "));
    std::io::stdout().flush().unwrap();
}

fn read(xx: &Vec<usize>) -> i8 {
    println!("? {}", join(xx.iter().map(|&x| x + 1), " "));
    let t = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<i8>().unwrap()
    };
    if t == -1 {
        std::process::exit(0);
    }

    t
}
