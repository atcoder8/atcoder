use itertools::join;

fn main() {
    let (n, k, _q): (usize, usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let mut aa: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let ll: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect()
    };

    for l in ll {
        if aa[l] == n {
            continue;
        }
        if l == k - 1 || aa[l] + 1 != aa[l + 1] {
            aa[l] += 1;
        }
    }

    println!("{}", join(aa.iter(), " "));
}
