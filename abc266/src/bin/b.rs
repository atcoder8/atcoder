const MOD: i64 = 998244353;

fn main() {
    let mut n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<i64>().unwrap()
    };

    n %= MOD;
    if n < 0 {
        n += MOD;
    }

    println!("{}", n);
}
