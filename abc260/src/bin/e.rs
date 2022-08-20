// unfinished

use itertools::join;

fn main() {
    let (n, m): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let mut ab: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        ab.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        });
    }

    let head = *ab.iter().map(|(a, _)| a).max().unwrap();
    let tail = *ab.iter().map(|(_, b)| b).min().unwrap();

    let mut ans = vec![];

    if head >= tail {
        let dist = tail - head + 1;

        for i in 1..(m + 1) {
            if i < dist {
                ans.push(0);
            } else {
                ans.push((i - dist + 1).min(m - i + 1));
            }
        }
    } else {
        for i in 1..(m + 1) {
            ans.push((m - i + 1).min(i + head - tail));
        }
    }

    println!("{}", join(ans.iter(), " "));
}
