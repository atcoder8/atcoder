use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};
use rand::Rng;

const MOD: usize = 1000000007;

fn main() {
    input! {
        (n, q): (usize, usize),
        aa: [Usize1; n],
        bb: [Usize1; n],
        queries: [(Usize1, usize, Usize1, usize); q],
    }

    let mut rng = rand::thread_rng();

    let hash_each_key = (0..n).map(|_| rng.gen_range(0..MOD)).collect_vec();

    let mut accumulated_hash_a = vec![0_usize; n + 1];
    for (i, &a) in enumerate(&aa) {
        accumulated_hash_a[i + 1] = (accumulated_hash_a[i] + hash_each_key[a]) % MOD;
    }

    let mut accumulated_hash_b = vec![0_usize; n + 1];
    for (i, &b) in enumerate(&bb) {
        accumulated_hash_b[i + 1] = (accumulated_hash_b[i] + hash_each_key[b]) % MOD;
    }

    for &(la, ra, lb, rb) in &queries {
        let hash_a = (accumulated_hash_a[ra] + MOD - accumulated_hash_a[la]) % MOD;
        let hash_b = (accumulated_hash_b[rb] + MOD - accumulated_hash_b[lb]) % MOD;
        println!("{}", if hash_a == hash_b { "Yes" } else { "No" });
    }
}
