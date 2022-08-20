use std::process;

fn main() {
    let y = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    for i in y.. {
        if i % 4 == 2 {
            println!("{}", i);
            process::exit(0);
        }
    }
}
