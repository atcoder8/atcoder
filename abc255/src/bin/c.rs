fn main() {
    let (x, a, d, n): (i64, i64, i64, i64) = {
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

    if d == 0 {
        println!("{}", (x - a).abs());
        std::process::exit(0);
    }

    let (min_elem, max_elem) = if d > 0 {
        (a, a + d * (n - 1))
    } else {
        (a + d * (n - 1), a)
    };

    if x <= min_elem {
        println!("{}", min_elem - x);
        std::process::exit(0);
    } else if x >= max_elem {
        println!("{}", x - max_elem);
        std::process::exit(0);
    }

    let cnt = (x - a).abs() / d.abs();
    let (elem1, elem2) = (a + d * cnt, a + d * (cnt + 1));
    let ans = std::cmp::min((x - elem1).abs(), (x - elem2).abs());
    println!("{}", ans);
}
