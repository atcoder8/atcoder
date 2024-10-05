use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        apbq: [(usize, usize, usize, usize); n],
    }

    let calc_min_sum_cost = |min_w: usize, machine1: Machine, machine2: Machine| {
        let calc_cost = |num1: usize, num2: usize| machine1.cost * num1 + machine2.cost * num2;

        let cand_costs_1 = (0..=machine2.speed).map(|num1| {
            let rem = min_w.saturating_sub(machine1.speed * num1);
            let num2 = ceil_div(rem, machine2.speed);
            calc_cost(num1, num2)
        });
        let cand_costs_2 = (0..=machine1.speed).map(|num2| {
            let rem = min_w.saturating_sub(machine2.speed * num2);
            let num1 = ceil_div(rem, machine1.speed);
            calc_cost(num1, num2)
        });

        cand_costs_1.chain(cand_costs_2).min().unwrap()
    };

    let is_ok = |min_w: usize| {
        let min_sum_cost = apbq
            .iter()
            .map(|&(speed1, cost1, speed2, cost2)| {
                let machine1 = Machine::new(speed1, cost1);
                let machine2 = Machine::new(speed2, cost2);
                calc_min_sum_cost(min_w, machine1, machine2)
            })
            .sum::<usize>();
        min_sum_cost <= x
    };

    let mut ok = 0_usize;
    let mut ng = 10_usize.pow(10);
    while ok.abs_diff(ng) > 1 {
        let mid = (ok + ng) / 2;
        if is_ok(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn ceil_div(numer: usize, denom: usize) -> usize {
    (numer + denom - 1) / denom
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    speed: usize,
    cost: usize,
}

impl Machine {
    fn new(speed: usize, cost: usize) -> Self {
        Self { speed, cost }
    }
}
