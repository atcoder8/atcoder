use proconio::input;

const DISTS: [usize; 6] = [3, 1, 4, 1, 5, 9];

fn main() {
    input! {
        (p, q): (char, char),
    }

    let mut p = p as u8 - b'A';
    let mut q = q as u8 - b'A';

    if p > q {
        std::mem::swap(&mut p, &mut q);
    }

    let ans: usize = (p..q).map(|i| DISTS[i as usize]).sum();
    println!("{}", ans);
}
