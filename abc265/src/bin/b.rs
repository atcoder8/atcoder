use std::process;

fn main() {
    let (n, m, t) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };
    let aa = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };
    let mut xy = Vec::new();
    for _ in 0..m {
        xy.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap() - 1,
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        });
    }

    let mut bonus = vec![0; n];
    for &(x, y) in xy.iter() {
        bonus[x] = y;
    }

    let mut rem = t;

    for i in 0..(n - 1) {
        rem += bonus[i];

        if rem > aa[i] {
            rem -= aa[i];
        } else {
            println!("No");
            process::exit(0);
        }
    }

    println!("Yes");
}
