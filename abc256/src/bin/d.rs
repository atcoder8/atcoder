fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let mut lr: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        lr.push({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        });
    }

    lr.sort_by_key(|x| x.0);

    let mut sections = vec![lr[0]];
    for (l, r) in lr.into_iter().skip(1) {
        let last_r = sections.last().unwrap().1;
        if l > last_r {
            sections.push((l, r));
        } else {
            if r > last_r {
                sections.last_mut().unwrap().1 = r;
            }
        }
    }

    sections.sort_by_key(|x| x.0);

    for (l, r) in sections {
        println!("{} {}", l, r);
    }
}
