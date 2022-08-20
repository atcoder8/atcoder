fn main() {
    let (l1, r1, l2, r2) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
            iter.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    let mut red = vec![false; 200];
    for i in l1..r1 {
        red[i] = true;
    }

    let mut blue = vec![false; 200];
    for i in l2..r2 {
        blue[i] = true;
    }

    let mut ans = 0;
    for i in 0..200 {
        if red[i] && blue[i] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
