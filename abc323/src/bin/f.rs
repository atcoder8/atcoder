use proconio::input;

type Coord = (i64, i64);

fn main() {
    input! {
        coord_a: (i64, i64),
        coord_b: (i64, i64),
        coord_c: (i64, i64),
    }

    let ans =
        strategy(coord_a, coord_b, coord_c, false).min(strategy(coord_a, coord_b, coord_c, true));
    println!("{}", ans);
}

fn strategy(mut coord_a: Coord, mut coord_b: Coord, mut coord_c: Coord, t: bool) -> i64 {
    if t {
        transposition(&mut coord_a, &mut coord_b, &mut coord_c);
    }

    let (move_x_cost, next_coord_a) = transport_x(coord_a, coord_b, coord_c);
    coord_a = next_coord_a;
    coord_b.0 = coord_c.0;

    transposition(&mut coord_a, &mut coord_b, &mut coord_c);

    let (move_y_cost, _) = transport_x(coord_a, coord_b, coord_c);

    move_x_cost + move_y_cost
}

fn reverse_x(coord_a: &mut Coord, coord_b: &mut Coord, coord_c: &mut Coord) {
    vec![coord_a, coord_b, coord_c]
        .into_iter()
        .for_each(|coord| coord.0 = -coord.0);
}

fn transposition(coord_a: &mut Coord, coord_b: &mut Coord, coord_c: &mut Coord) {
    vec![coord_a, coord_b, coord_c]
        .into_iter()
        .for_each(|coord| std::mem::swap(&mut coord.0, &mut coord.1));
}

fn transport_x(mut coord_a: Coord, mut coord_b: Coord, mut coord_c: Coord) -> (i64, Coord) {
    if coord_b.0 == coord_c.0 {
        return (0, coord_a);
    }

    if coord_a.0 > coord_b.0 {
        reverse_x(&mut coord_a, &mut coord_b, &mut coord_c);
        let (cost, mut next_coord_a) = transport_x(coord_a, coord_b, coord_c);
        next_coord_a.0 = -next_coord_a.0;

        return (cost, next_coord_a);
    }

    if coord_b.0 < coord_c.0 {
        let cost = (coord_b.1 - coord_a.1).abs() + coord_c.0 - 1 - coord_a.0
            + 2 * (coord_a.0 == coord_b.0) as i64;
        let next_coord_a = (coord_c.0 - 1, coord_b.1);

        return (cost, next_coord_a);
    }

    let first_half_cost = coord_b.0 + 1 - coord_a.0 + 2 * (coord_a.1 == coord_b.1) as i64;
    coord_a.0 = coord_b.0 + 1;
    reverse_x(&mut coord_a, &mut coord_b, &mut coord_c);
    let (second_half_cost, mut next_coord_a) = transport_x(coord_a, coord_b, coord_c);
    next_coord_a.0 = -next_coord_a.0;

    (first_half_cost + second_half_cost, next_coord_a)
}
