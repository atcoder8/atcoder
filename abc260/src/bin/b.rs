use std::cmp::Reverse;

fn main() {
    let (n, x, y, z): (usize, usize, usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let aa: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let bb: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    let mut iab: Vec<(usize, usize, usize)> = vec![];
    for i in 0..n {
        iab.push((i, aa[i], bb[i]));
    }

    let mut ans = vec![];

    iab.sort_by_key(|x| Reverse(x.0));
    iab.sort_by_key(|x| x.1);
    for _ in 0..x {
        ans.push(iab.pop().unwrap());
    }

    iab.sort_by_key(|x| Reverse(x.0));
    iab.sort_by_key(|x| x.2);
    for _ in 0..y {
        ans.push(iab.pop().unwrap());
    }

    iab.sort_by_key(|x| Reverse(x.0));
    iab.sort_by_key(|x| x.1 + x.2);
    for _ in 0..z {
        ans.push(iab.pop().unwrap());
    }

    ans.sort_by_key(|x| x.0);

    ans.iter().for_each(|(i, _, _)| println!("{}", i + 1));
}
