#![allow(unused)]

fn main() {
    // ax = c (mod b) has solution
    //   x = x0 + k b/g where 
    //   g = gcd(a, b)
    //   x0 = minv(a / g, b / g) * c/g
    // if g divides c

    // ax + by = c has solution
    //   x = c/g x0 + k b/g where k is int
    //   y = c/g y0 + k a/g where k is int
    //   x0, y0 is the result of extgcd(a, b)
    // if g divides c

    let tc = read::<usize>();
    let mut ans = vec![];
    for t in 0..tc {
        let inp = readv::<i64>();
        let (n, s, k) = (inp[0], inp[1], inp[2]);

        // s + kx = 0 (mod n)
        // kx = -s (mod n) has solution:
        // x = x0 + k b/g where k is int, x0 = minv(k / g, n / g) * (-s / g)
        // To find the minimum x such that x >= 0, 
        // xmin = x0 mod b/g
        let g = gcd(k, n);
        if (-s).rem_euclid(g) != 0 {
            ans.push(-1);
        } else {
            let x0 = minv(k / g, n / g) * (-s) / g;
            let x = x0.rem_euclid(n / g);
            ans.push(x);
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

// extgcd finds a (x0, y0) given (a, b) in equation
// ax + by = g

// In gcd, we have
// gcd(a, b) = gcd(b, a mod b)
// that is,
// g = a * x0 + b y0             (1)
// g = b * x1 + (a mod b) y1     (2)
// ...
// g = g * 1 + 0 * 0

// if we know (2), then (1) can be found using:
// g = b * x1 + (a mod b) y1
// g = b * x1 + (a - floor(a / b) * b) y1
// g = a * y1 + b (x1 - y1 * floor(a / b))
// that is,
// x0 = y1
// y0 = x1 - y1 * floor(a / b)

fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (1, 0, a) // (x0, y0, g)
    } else {
        let (x1, y1, g) = extgcd(b, a.rem_euclid(b));
        (y1, x1 - y1 * (a / b), g) // (x0, y0, g)
    }
}

// Find the x of ax = 1 (mod m)
fn minv(a: i64, m: i64) -> i64 {
    let (x0, _, _) = extgcd(a, m);
    x0.rem_euclid(m)
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
