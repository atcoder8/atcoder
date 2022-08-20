use std::collections::HashSet;

fn main() {
    let n: i64 = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let mut s = HashSet::new();
    for _ in 0..n {
        let aa: Vec<i64> = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        };
        s.insert(aa.clone());
    }

    println!("{}", s.len());
}
