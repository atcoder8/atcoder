use std::ops;

use itertools::Itertools;
use proconio::input;

const EPSILON: f64 = 1e-10;

fn main() {
    input! {
        t: usize,
    }

    let output = (0..t).map(|_| solve()).join("\n");
    println!("{output}");
}

fn solve() -> f64 {
    input! {
        (a, b): ([f64; 2], [f64; 2]),
        (c, d): ([f64; 2], [f64; 2]),
    }

    let mut a = Vector2D([a[0], a[1]]);
    let mut b = Vector2D([b[0], b[1]]);
    let mut c = Vector2D([c[0], c[1]]);
    let mut d = Vector2D([d[0], d[1]]);

    if (b - a).norm() < (d - c).norm() {
        std::mem::swap(&mut a, &mut c);
        std::mem::swap(&mut b, &mut d);
    }

    let ratio = (d - c).norm() / (b - a).norm();
    let e = (1.0 - ratio) * a + ratio * b;

    let f = c - a;
    let g = d - e;

    let cand1 = if (f - g).norm() >= EPSILON {
        dist_point_and_line([0.0; 2], [f.0, g.0])
    } else {
        f.norm()
    };
    let cand2 = if (b - e).norm() >= EPSILON {
        dist_point_and_line(d.0, [e.0, b.0])
    } else {
        (e - d).norm()
    };
    cand1.min(cand2)
}

#[derive(Debug, Clone, Copy)]
struct Vector2D([f64; 2]);

impl ops::Add for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl ops::Sub for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl ops::Mul<f64> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0.map(|x| x * rhs))
    }
}

impl ops::Mul<Vector2D> for f64 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        rhs * self
    }
}

impl Vector2D {
    fn norm(&self) -> f64 {
        self.0[0].hypot(self.0[1])
    }
}

/// 直線の方程式の一般形`ax+by+c=0`を表します。
#[derive(Debug, Clone, Copy)]
struct StraightLine {
    a: f64,
    b: f64,
    c: f64,
}

impl StraightLine {
    fn from_two_points(coord1: [f64; 2], coord2: [f64; 2]) -> Self {
        let (x1, y1) = (coord1[0], coord1[1]);
        let (x2, y2) = (coord2[0], coord2[1]);
        Self {
            a: y1 - y2,
            b: x2 - x1,
            c: x1 * y2 - x2 * y1,
        }
    }

    fn vertical_line(&self, passing_point: [f64; 2]) -> Self {
        let Self { a, b, c: _ } = *self;
        let [x0, y0] = passing_point;
        Self {
            a: b,
            b: -a,
            c: -b * x0 + a * y0,
        }
    }

    fn intersection(&self, other: &Self) -> [f64; 2] {
        let Self {
            a: a11,
            b: a12,
            c: c1,
        } = *self;
        let b1 = -c1;

        let Self {
            a: a21,
            b: a22,
            c: c2,
        } = *other;
        let b2 = -c2;

        let denom = a11 * a22 - a12 * a21;
        [(a22 * b1 - a12 * b2) / denom, (a11 * b2 - a21 * b1) / denom]
    }
}

fn dist_point_and_line(point: [f64; 2], line_segment: [[f64; 2]; 2]) -> f64 {
    let p = Vector2D(point);
    let [p1, p2] = line_segment.map(Vector2D);
    let line = StraightLine::from_two_points(p1.0, p2.0);
    let vertical_line = line.vertical_line(p.0);
    let h = line.intersection(&vertical_line);

    let [x1, x2]: [f64; 2] = std::array::from_fn(|i| line_segment[i][0]);
    let [y1, y2]: [f64; 2] = std::array::from_fn(|i| line_segment[i][1]);
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    if (min_x - EPSILON..max_x + EPSILON).contains(&h[0])
        && (min_y - EPSILON..max_y + EPSILON).contains(&h[1])
    {
        (Vector2D(h) - p).norm()
    } else {
        (p1 - p).norm().min((p2 - p).norm())
    }
}
