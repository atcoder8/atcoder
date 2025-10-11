use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let calc_min_cost = |bits: usize| {
        uv.iter()
            .filter(move |&&(u, v)| (bits >> u & 1) == (bits >> v & 1))
            .count()
    };

    let min_cost = (0..1 << n).map(|bits| calc_min_cost(bits)).min().unwrap();
    println!("{min_cost}");
}
