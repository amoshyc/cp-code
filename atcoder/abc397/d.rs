#![allow(unused)]

fn main() {
    let n = read::<u64>();

    // x^3 - y^3 = n
    // (x - y)(x^2 + xy + y^2) = n
    // let x = y + d (d > 0)
    // d ((y + d)^2 + (y + d)y + y^2) = n
    // d (y^2 + 2yd + d^2 + y^2 + dy + y^2) = n
    // d (3y^2 + 3yd + d^2) = n

    // => d^3 <= n
    // => enumerate d and binary search y

    let f = |d: u64, y: u64| -> u64 {
        let item0 = y.saturating_mul(y).saturating_mul(3);
        let item1 = y.saturating_mul(d).saturating_mul(3);
        let item2 = d.saturating_mul(d);
        d.saturating_mul(item0.saturating_add(item1).saturating_add(item2))
    };

    for d in (1u64..).take_while(|d| d.saturating_pow(3) <= n) {
        // 0 0 0 1 1 1
        let mut lb = 0;
        let mut ub = n;
        while ub - lb > 1 {
            let m = (lb + ub) / 2;
            if f(d, m) < n {
                lb = m;
            } else {
                ub = m;
            }
        }

        if f(d, ub) == n {
            println!("{} {}", ub + d, ub);
            return;
        }
    }

    println!("-1");
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
