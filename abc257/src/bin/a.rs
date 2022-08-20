use std::char::from_u32;

fn main() {
    let (n, x): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let mut chars: Vec<char> = vec![];
    for i in 0..26 {
        (0..n).for_each(|_| chars.push(from_u32('A' as u32 + i).unwrap()));
    }

    println!("{}", chars[x - 1]);
}
