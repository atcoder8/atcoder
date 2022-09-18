use std::io::{self, Write};

use std::collections::HashMap;

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    // ルークの置かれていない行

    let mut memory: HashMap<(usize, usize), usize> = HashMap::new();

    let mut ok: usize = n;
    let mut ng: usize = 0;

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;

        let t = if let Some(&t) = memory.get(&(mid, n)) {
            t
        } else {
            println!("? 1 {} 1 {}", mid, n);
            io::stdout().flush().unwrap();

            let t = {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line.trim().parse::<usize>().unwrap()
            };

            memory.insert((mid, n), t);

            t
        };

        if t < mid {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let row = ok;

    let mut ok: usize = n;
    let mut ng: usize = 0;

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;

        let t = if let Some(&t) = memory.get(&(n, mid)) {
            t
        } else {
            println!("? 1 {} 1 {}", n, mid);
            io::stdout().flush().unwrap();

            let t = {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line.trim().parse::<usize>().unwrap()
            };

            memory.insert((n, mid), t);

            t
        };

        if t < mid {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    let col = ok;

    println!("! {} {}", row, col);
    io::stdout().flush().unwrap();
}
