use std::collections::BTreeMap;

fn main() {
    let (n, k): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let pp: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect()
    };

    let mut eat: Vec<Option<usize>> = vec![None; n];
    let mut front: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

    for i in 0..n {
        let p = pp[i];
        match front.range(p..).next() {
            Some((&top, _)) => {
                let mut stack = front.remove(&top).unwrap();
                stack.push(p);
                if stack.len() == k {
                    stack.iter().for_each(|x| eat[*x] = Some(i));
                } else {
                    front.insert(p, stack);
                }
            },
            None => {
                if k == 1 {
                    eat[p] = Some(i);
                } else {
                    front.insert(p, vec![p]);
                }
            },
        }
    }

    eat.iter().for_each(|x| match x {
        Some(x) => println!("{}", x + 1),
        None => println!("-1"),
    })
}
