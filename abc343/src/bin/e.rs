use itertools::{iproduct, izip, Itertools};
use proconio::input;

const N: usize = 7;

fn main() {
    match solve() {
        Some((shift1, shift2)) => println!(
            "Yes\n{N} {N} {N} {} {} {} {} {} {}",
            shift1.0, shift1.1, shift1.2, shift2.0, shift2.1, shift2.2
        ),
        None => println!("No"),
    }
}

fn solve() -> Option<((usize, usize, usize), (usize, usize, usize))> {
    input! {
        vv: [usize; 3],
    }

    let is_match = |shift1: (usize, usize, usize), shift2: (usize, usize, usize)| {
        let cube0 = Rect::new((N, N, N));
        let cube1 = Rect::new(shift1);
        let cube2 = Rect::new(shift2);

        let cube01 = cube0.intersection(&cube1);
        let cube02 = cube0.intersection(&cube2);
        let cube12 = cube1.intersection(&cube2);

        let cube123 = cube01.intersection(&cube2);

        let volume3 = cube123.volume();
        let volume2 = cube01.volume() + cube02.volume() + cube12.volume() - 3 * volume3;
        let volume1 = cube0.volume() + cube1.volume() + cube2.volume() - 2 * volume2 - 3 * volume3;

        volume1 == vv[0] && volume2 == vv[1] && volume3 == vv[2]
    };

    for shift1 in iproduct!(0..=N, 0..=N, 0..=N) {
        for shift2 in iproduct!(0..=2 * N, 0..=2 * N, 0..=2 * N) {
            if is_match(shift1, shift2) {
                return Some((shift1, shift2));
            }
        }
    }

    None
}

#[derive(Debug, Clone)]
struct Rect {
    min_coords: Vec<usize>,
    max_coords: Vec<usize>,
}

impl Rect {
    fn new(shift: (usize, usize, usize)) -> Self {
        let min_coords = vec![shift.0, shift.1, shift.2];
        let max_coords = vec![shift.0 + N, shift.1 + N, shift.2 + N];

        Rect {
            min_coords,
            max_coords,
        }
    }

    fn intersection(&self, other: &Rect) -> Self {
        let min_coords = izip!(&self.min_coords, &other.min_coords)
            .map(|(&min1, &min2)| min1.max(min2))
            .collect_vec();
        let max_coords = izip!(&self.max_coords, &other.max_coords)
            .map(|(&max1, &max2)| max1.min(max2))
            .collect_vec();

        if izip!(&min_coords, &max_coords).any(|(min, max)| min >= max) {
            return Self {
                min_coords: vec![0, 0, 0],
                max_coords: vec![0, 0, 0],
            };
        }

        Self {
            min_coords,
            max_coords,
        }
    }

    fn volume(&self) -> usize {
        izip!(&self.min_coords, &self.max_coords)
            .map(|(min, max)| max - min)
            .product()
    }
}
