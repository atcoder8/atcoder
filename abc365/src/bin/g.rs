use im_rc::HashMap;
use itertools::{enumerate, Itertools};
use num_integer::Roots;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        tp: [(usize, Usize1); m],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }

    let mut intervals = vec![vec![]; n];
    let mut entry_times = vec![None; n];
    for &(t, p) in &tp {
        match entry_times[p].take() {
            Some(entry_time) => intervals[p].push((entry_time, t)),
            None => entry_times[p] = Some(t),
        }
    }

    let calc_common_times = |target_stuff: usize| {
        // 各スタッフについて、`target_stuff`が入室している時間のうち、入室している時間もしくは退室している時間の長さを管理する
        // 初期状態は入室している時間の長さを表し、入室または退室するごとに入室時間、退室時間のうちどちらを管理しているかが入れ替わる
        // 各スタッフについて偶数回入れ替わるため、最終的には`target_stuff`と同時に入室していた時間が格納される
        let mut common_or_uncommon_times = vec![0_usize; n];

        let mut entered_time = 0;
        let mut entered = false;
        let mut prev_time = 0;
        for &(t, p) in &tp {
            let extended_entered_time = entered_time + if entered { t - prev_time } else { 0 };

            if p == target_stuff {
                entered_time = extended_entered_time;
                entered = !entered;
                prev_time = t;
            }

            common_or_uncommon_times[p] = extended_entered_time - common_or_uncommon_times[p];
        }

        common_or_uncommon_times
    };

    let size_boundary = m.sqrt();

    let mut common_times_map = HashMap::<(usize, usize), usize>::new();
    for target_stuff in 0..n {
        if intervals[target_stuff].len() > size_boundary {
            let common_times = calc_common_times(target_stuff);
            common_times_map.extend(enumerate(common_times).flat_map(|(other_stuff, time)| {
                [
                    ((target_stuff, other_stuff), time),
                    ((other_stuff, target_stuff), time),
                ]
            }));
        }
    }

    let solve = |a: usize, b: usize| {
        if let Some(&common_time) = common_times_map.get(&(a, b)) {
            return common_time;
        }

        let interval_a = &intervals[a];
        let interval_b = &intervals[b];

        let mut common_times = 0;
        for &(start_a, end_a) in interval_a {
            let mut b_idx = 0;
            loop {
                b_idx += interval_b[b_idx..].partition_point(|&(_, end_b)| end_b <= start_a);
                if b_idx == interval_b.len() {
                    break;
                }

                let (start_b, end_b) = interval_b[b_idx];
                if start_b >= end_a {
                    break;
                }

                common_times += end_a.min(end_b).saturating_sub(start_a.max(start_b));
                b_idx += 1;
            }
        }

        common_times
    };

    let answer = ab.iter().map(|&(a, b)| solve(a, b)).join("\n");
    println!("{}", answer);
}
