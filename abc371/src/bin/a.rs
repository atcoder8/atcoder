use itertools::izip;
use proconio::input;

fn main() {
    input! {
        ss: [char; 3],
    }

    let mut counts = [0; 3];
    for (&s, (i, j)) in izip!(&ss, [(0, 1), (0, 2), (1, 2)]) {
        if s == '<' {
            counts[i] += 1;
        } else {
            counts[j] += 1;
        }
    }

    let pos = counts.iter().position(|&cnt| cnt == 1).unwrap();
    let ans = (b'A' + pos as u8) as char;
    println!("{}", ans);
}
