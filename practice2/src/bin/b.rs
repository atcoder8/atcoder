fn main() {
    // let a = 37;
    // let b = 10;
    // println!("{} / {} = {} ... {}", a, b, a / b, a % b);
    // println!("{} / {} = {} ... {}", a, -b, a / (-b), a % (-b));
    // println!("{} / {} = {} ... {}", -a, b, (-a) / b, (-a) % b);
    // println!("{} / {} = {} ... {}", -a, -b, (-a) / (-b), (-a) % (-b));

    for a in -100..100 {
        modinv(a, 1000000007);
        for b in -100..100 {
            ext_gcd(a, b);
            // let ((x, y), g) = ext_gcd(a, b);
            // println!("{} * {} + {} * {} = {}\n", a, x, b, y, g);
        }
    }

    // let (a, b) = (1000000006, 3);
    // let ((x, y), g) = ext_gcd(a, b);
    // println!("{} * {} + {} * {} = {}", a, x, b, y, g);
}

fn ext_gcd(mut a: i64, mut b: i64) -> ((i64, i64), i64) {
    let c = std::cmp::max(a.abs(), b.abs());
    let (mut s, mut t, mut u, mut v) = (1, 0, 0, 1);
    while b != 0 {
        let q = a / b;
        a -= q * b;
        std::mem::swap(&mut a, &mut b);
        s -= q * t;
        std::mem::swap(&mut s, &mut t);
        u -= q * v;
        std::mem::swap(&mut u, &mut v);

        // println!("(a, b, s, t, u, v, q) = ({}, {}, {}, {}, {}, {}, {})", a, b, s, t, u, v, q);
        if *vec![a, b, s, t, u, v, q].iter().max().unwrap() > c {
            println!("/// unexpected! ///");
        }
    }

    ((s, u), a)
}

#[allow(dead_code)]
fn modinv(mut a: i64, m: i64) -> i64 {
    assert!(m >= 2);

    let (mut b, mut s, mut t) = (m, 1, 0);
    while b != 0 {
        let q = a / b;
        a -= q * b;
        std::mem::swap(&mut a, &mut b);
        s -= q * t;
        std::mem::swap(&mut s, &mut t);

        // println!("a = {}, b = {}, s = {}, t = {}, q = {}", a, b, s, t, q);
    }

    assert_eq!(a.abs(), 1);

    s %= m;
    if s < 0 {
        s += m;
    }

    s
}
