#![allow(unused)]

fn main() {
    let inf = std::i64::MAX;
    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        let inp = readv::<i64>();
        let (x, y, p, q) = (inp[0], inp[1], inp[2], inp[3]);

        let mut res = inf;
        for t1 in p..(p + q) {
            for t2 in x..(x + y) {
                let (r1, r2) = (t1, t2);
                let (m1, m2) = (p + q, 2 * x + 2 * y);
                if let Some((lcm, x)) = crt(&vec![(m1, r1), (m2, r2)]) {
                    res = res.min(x);
                }
            }
        }

        if res == inf {
            ans.push("infinity".to_string());
        } else {
            ans.push(format!("{}", res));
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a.rem_euclid(b))
    }
}

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (1, 0, a) // (x0, y0, g)
    } else {
        let (x1, y1, g) = extgcd(b, a.rem_euclid(b));
        (y1, x1 - y1 * (a / b), g) // (x0, y0, g)
    }
}

fn minv(a: i64, m: i64) -> i64 {
    let (x0, _, _) = extgcd(a, m);
    x0.rem_euclid(m)
}

// ax = b (mod m) has solution
//  x = (a/g)^(-1) * (b/g) (mod m/g)
fn linear_congruence(a: i64, b: i64, m: i64) -> Option<(i64, i64)> {
    let (inv, _, g) = extgcd(a, m);
    if b % g != 0 {
        None
    } else {
        Some(((inv * (b / g)).rem_euclid(m / g), g))
    }
}

fn crt(coef: &Vec<(i64, i64)>) -> Option<(i64, i64)> {
    let mut m1 = coef[0].0;
    let mut r1 = coef[0].1;
    for i in 1..coef.len() {
        let (m2, r2) = coef[i];
        if let Some((q1, g)) = linear_congruence(m1, r2 - r1, m2) {
            let lcm = m1 / g * m2;
            r1 = (m1 * q1 + r1).rem_euclid(lcm);
            m1 = lcm;
        } else {
            return None;
        }
    }
    Some((m1, r1))
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
