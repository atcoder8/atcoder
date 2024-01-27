use itertools::enumerate;
use proconio::{input, marker::Usize1};

fn main() {
    println!("{}", if solve() { "Yes" } else { "No" });
}

fn solve() -> bool {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    }

    let mut chords = vec![(0, 0); 2 * n];
    for (chord, &(a, b)) in enumerate(&ab) {
        chords[a] = (b, chord);
        chords[b] = (a, chord);
    }

    let mut visited = vec![false; 2 * n];
    let mut chord_stack: Vec<usize> = vec![];
    for cur in 0..2 * n {
        let (opposite, chord) = chords[cur];

        if visited[cur] {
            let last_chord = chord_stack.pop().unwrap();

            if chord != last_chord {
                return true;
            }

            continue;
        }

        visited[cur] = true;
        visited[opposite] = true;
        chord_stack.push(chord);
    }

    false
}
