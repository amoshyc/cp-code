#![allow(unused)]

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];
    for t in 0..tc {
        let inp = readv::<i64>();
        let (n, s, k) = (inp[0], inp[1], inp[2]);

        // s + kx = 0 (mod n)
        // kx = -s (mod n)
        ans.push(linear_congruence(k, -s, n).unwrap_or(-1));
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
fn linear_congruence(a: i64, b: i64, m: i64) -> Option<i64> {
    let (inv, _, g) = extgcd(a, m);
    if b % g != 0 {
        None
    } else {
        Some((inv * (b / g)).rem_euclid(m / g))
    }
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
