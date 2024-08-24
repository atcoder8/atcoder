use itertools::{enumerate, Itertools};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, x0): (usize, usize, usize),
        abst: [(Usize1, Usize1, usize, usize); m],
    }

    let events = enumerate(&abst)
        .flat_map(|(idx, &(a, b, s, t))| {
            [
                Event {
                    from: a,
                    to: b,
                    time: s,
                    leave: true,
                    idx,
                },
                Event {
                    from: a,
                    to: b,
                    time: t,
                    leave: false,
                    idx,
                },
            ]
        })
        .sorted_unstable_by_key(|event| (event.time, event.leave))
        .collect_vec();

    let mut delay_times = vec![0_usize; m];
    delay_times[0] = x0;
    let mut arrival_times = vec![0_usize; n];
    for &event in &events {
        let Event {
            from,
            to,
            time,
            leave,
            idx,
        } = event;

        if leave {
            if idx != 0 {
                delay_times[idx] = arrival_times[from].saturating_sub(time);
            }
        } else {
            arrival_times[to] = arrival_times[to].max(time + delay_times[idx]);
        }
    }

    println!("{}", delay_times[1..].iter().join(" "));
}

#[derive(Debug, Clone, Copy)]
struct Event {
    from: usize,
    to: usize,
    time: usize,
    leave: bool,
    idx: usize,
}
