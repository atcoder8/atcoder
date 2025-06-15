use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        match solve() {
            Some((x1, x2)) => println!("Yes\n{x1} {x2}"),
            None => println!("No"),
        }
    }
}

fn solve() -> Option<(usize, usize)> {
    input! {
        (mut a1, mut a2, a3): (u32, u32, u32),
    }

    let mut swapped = false;
    if a1 > a2 {
        std::mem::swap(&mut a1, &mut a2);
        swapped = true;
    }

    if a3 < a2 || a3 > a1 + a2 {
        return None;
    }

    let raised1 = 10_usize.pow(a1 - 1);
    let raised2 = 10_usize.pow(a2 - 1);

    let (mut x1, mut x2) = if a2 == a3 {
        (raised1, raised2)
    } else if a1 + a2 == a3 {
        (
            5 * raised1,
            "3".repeat(a2 as usize).parse::<usize>().unwrap(),
        )
    } else {
        let diff = a3 - a2;
        (raised1 + 10_usize.pow(a1 - 1 - diff), raised2)
    };

    if swapped {
        std::mem::swap(&mut x1, &mut x2);
    }

    Some((x1, x2))
}
