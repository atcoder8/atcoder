fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };
    let cc: Vec<char> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().chars().collect()
    };
    let ww: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
    };

    let mut wc: Vec<(usize, char)> = ww.iter().zip(cc.iter()).map(|(w, c)| (*w, *c)).collect();
    wc.sort();
    let mut adult = vec![0];
    for i in 0..n {
        adult.push(adult[i] + if wc[i].1 == '1' { 1 } else { 0 });
    }

    let mut ans = std::cmp::max(*adult.last().unwrap(), n - adult.last().unwrap());
    for i in 0..n {
        if i != 0 && wc[i].0 == wc[i - 1].0 {
            continue;
        }
        let cnt = i - adult[i] + adult[n] - adult[i];
        ans = ans.max(cnt);
    }

    println!("{}", ans);
}
