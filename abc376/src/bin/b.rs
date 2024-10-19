use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, q): (usize, usize),
        ht: [(char, Usize1); q],
    }

    let mut ans = 0_usize;
    let mut positions_per_hand: [usize; 2] = [0, 1];
    for &(h, t) in &ht {
        let hand: usize = if h == 'L' { 0 } else { 1 };

        let count_rotate_times = |dist: usize| {
            let mut cnt = 0_usize;
            let mut pos = positions_per_hand[hand];
            while pos != t {
                pos = (pos + dist) % n;
                cnt += 1;

                if pos == positions_per_hand[1 - hand] {
                    return None;
                }
            }

            Some(cnt)
        };

        ans += count_rotate_times(1).or(count_rotate_times(n - 1)).unwrap();

        positions_per_hand[hand] = t;
    }

    println!("{}", ans);
}
