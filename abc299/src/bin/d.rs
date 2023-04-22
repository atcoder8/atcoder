use std::io::Write;

fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };

    let is_ok = |x: usize| {
        println!("? {}", x + 1);
        std::io::stdout().flush().unwrap();
        let s = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<usize>().unwrap()
        };

        s == 1
    };

    let ans = {
        let mut ok = n - 1;
        let mut ng = 0;

        while ok - ng > 1 {
            let mid = (ok + ng) / 2;

            if is_ok(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    };

    println!("! {}", ans);
}
