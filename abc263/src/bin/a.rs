fn main() {
    if solve() {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve() -> bool {
    let mut aa = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };
    aa.sort_unstable();

    (aa[0] == aa[1] && aa[1] != aa[2] && aa[2] == aa[3] && aa[3] == aa[4])
        || (aa[0] == aa[1] && aa[1] == aa[2] && aa[2] != aa[3] && aa[3] == aa[4])
}
