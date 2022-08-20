fn main() {
    let (a, b, d): (f64, f64, f64) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let (sin, cos) = d.to_radians().sin_cos();
    let (a2, b2) = (cos * a - sin * b, sin * a + cos * b);

    println!("{} {}", a2, b2);
}
