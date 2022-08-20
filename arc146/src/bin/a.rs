use itertools::{Itertools, join};

fn main() {
    let _n = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse::<usize>().unwrap()
    };
    let mut aa = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };
    aa.sort_unstable();
    aa.reverse();

    let cards = aa[..3].to_vec();
    let mut ans = 0;
    
    for perm in cards.iter().permutations(3) {
        let num: usize = join(perm, "").parse().unwrap();
        ans = ans.max(num);
    }

    println!("{}", ans);
}
