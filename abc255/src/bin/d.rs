fn main() {
    let (n, q): (usize, usize) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };
    let mut aa: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };
    let mut xx: Vec<usize> = Vec::new();
    for _ in 0..q {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        xx.push(line.trim().parse().unwrap());
    }

    aa.sort_unstable();

    let mut cumsum = vec![0];
    for i in 0..n {
        cumsum.push(cumsum[i] + aa[i]);
    }

    for x in xx {
        let small_num = match aa.binary_search(&x) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        let ans = x * small_num - cumsum[small_num] + cumsum[n] - cumsum[small_num] - x * (n - small_num);
        println!("{}", ans);
    }
}
