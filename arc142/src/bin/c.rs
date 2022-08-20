fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap()
    };

    let mut dist1 = vec![0; n];
    let mut dist2 = vec![0; n];

    for i in 2..n {
        println!("? 1 {}", i + 1);
        let d: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse().unwrap()
        };
        dist1[i] = d;

        println!("? 2 {}", i + 1);
        let d: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse().unwrap()
        };
        dist2[i] = d;
    }

    let min_dist = (2..n).map(|i| dist1[i] + dist2[i]).min().unwrap();
    if min_dist != 3 {
        println!("! {}", min_dist);
        std::process::exit(0);
    }

    let mut a = n;
    let mut b = n;
    for i in 2..n {
        if dist1[i] == 1 && dist2[i] == 2 {
            a = i;
        }
        if dist1[i] == 2 && dist2[i] == 1 {
            b = i;
        }
    }
    if a == n || b == n {
        println!("! 1");
    } else {
        println!("? {} {}", a + 1, b + 1);
        let d: usize = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse().unwrap()
        };
        if d == 1 {
            println!("! 3");
        } else {
            println!("! 1");
        }
    }
}
