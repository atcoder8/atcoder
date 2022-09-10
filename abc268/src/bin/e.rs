fn main() {
    let n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let pp = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    let mut l = 0;
    let mut r = 2 * n;

    while l + 2 < r {
        let c1 = l + (r - l) / 3;
        let c2 = r - (r - l) / 3;

        let total_cost_1: usize = pp.iter().enumerate().map(|(i, &p)| ((i + c1 + n - p) % n).min((p + 10 * n - (i + c1)) % n)).sum();
        let total_cost_2: usize = pp.iter().enumerate().map(|(i, &p)| ((i + c2 + n - p) % n).min((p + 10 * n - (i + c2)) % n)).sum();

        if total_cost_1 < total_cost_2 {
            r = c2;
        } else {
            l = c1;
        }
    }

    let mut ans: usize = pp.iter().enumerate().map(|(i, &p)| ((i + l + n - p) % n).min((p + 3 * n - (i + l)) % n)).sum();
    ans = ans.min(pp.iter().enumerate().map(|(i, &p)| ((i + l + 1 + n - p) % n).min((p + 10 * n - (i + l + 1)) % n)).sum());
    ans = ans.min(pp.iter().enumerate().map(|(i, &p)| ((i + l + 2 + n - p) % n).min((p + 10 * n - (i + l + 2)) % n)).sum());

    println!("{}", ans);
}
