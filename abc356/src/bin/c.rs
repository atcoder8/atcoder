use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        aar: [([Usize1], char); m],
    }

    let is_ok = |pattern: usize| {
        for (aa, r) in &aar {
            let open = aa.iter().filter(|&a| pattern >> a & 1 == 1).count() >= k;
            if open != (*r == 'o') {
                return false;
            }
        }

        true
    };

    let ans = (0..1 << n).filter(|&pattern| is_ok(pattern)).count();
    println!("{}", ans);
}
