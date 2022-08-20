fn main() {
    let (_n, x, y): (usize, usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
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

    let mut bb: Vec<usize> = aa.iter().map(|a| a % (x + y)).collect();

    if bb.iter().all(|b| *b < x) {
        println!("Second");
        std::process::exit(0);
    }

    if x <= y {
        println!("First");
    } else {
        bb.iter_mut().for_each(|b| {
            if *b >= x {
                *b -= x;
            }
        });
        if bb.iter().all(|b| *b < y) {
            println!("First");
        } else {
            println!("Second");
        }
    }
}
