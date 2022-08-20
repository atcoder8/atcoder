fn main() {
    let (a, b, c): (i64, i64, i64) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let (x, y, z) = (a + b - c, a - b + c, -a + b + c);
    let min_xyz = *vec![x, y, z].iter().min().unwrap();
    if min_xyz < 0 {
        println!("-1");
        std::process::exit(0);
    }

    let ans = min_xyz + (x + y + z - 3 * min_xyz) / 2;
    println!("{}", ans);
}
