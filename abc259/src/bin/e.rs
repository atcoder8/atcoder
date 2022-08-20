use std::collections::HashMap;

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };

    let mut mm = vec![];
    let mut pepe = vec![];

    for _ in 0..n {
        let m: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse().unwrap()
        };
        mm.push(m);

        let mut pe: Vec<(usize, usize)> = Vec::new();
        for _ in 0..m {
            pe.push({
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let mut iter = line.split_whitespace();
                (
                    iter.next().unwrap().parse().unwrap(),
                    iter.next().unwrap().parse().unwrap(),
                )
            });
        }
        pepe.push(pe);
    }

    let mut hm: HashMap<usize, (usize, bool)> = HashMap::new();
    for pe in pepe.iter() {
        for (p, e) in pe.iter() {
            match hm.get_mut(p) {
                Some((max_e, multi_flag)) => {
                    if e == max_e {
                        *multi_flag = true;
                    } else if e > max_e {
                        *max_e = *e;
                        *multi_flag = false;
                    }
                }
                None => {
                    hm.insert(*p, (*e, false));
                }
            }
        }
    }

    let mut ans = 0;

    let mut full_flag = false;
    for pe in pepe {
        let mut change_flag = false;

        for (p, e) in pe {
            if hm.get(&p) == Some(&(e, false)) {
                change_flag = true;
                break;
            }
        }

        if change_flag {
            ans += 1;
        } else if !full_flag {
            ans += 1;
            full_flag = true;
        }
    }

    println!("{}", ans);
}
