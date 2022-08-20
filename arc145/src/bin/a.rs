const A: char = 'A';
const B: char = 'B';

fn main() {
    if solve() {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve() -> bool {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let cc: Vec<char> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().collect()
    };

    if n == 2 {
        return cc == vec![A, A] || cc == vec![B, B];
    }

    return (cc[0], cc[n - 1]) != (A, B);
}
