#![allow(unused)]

// euclidean_div is the same as floor_div.
// The remainder is always >= 0.
// a/b (b > 0) rounds toward zero.
// when a < 0, we left shift the value if needed.
// when a >= 0, a/b is what we want.
fn euclidean_div(mut a: i64, mut b: i64) -> (i64, i64) {
    if b < 0 {
        a = -a;
        b = -b;
    }
    let mut q = a / b;
    if q < 0 && a % b != 0 {
        q -= 1;
    }
    (q, a - b * q)
}

fn floor_div(a: i64, b: i64) -> i64 {
    let (q, r) = euclidean_div(a, b);
    q
}

fn ceil_div(a: i64, b: i64) -> i64 {
    let (q, r) = euclidean_div(a, b);
    q + if r != 0 { 1 } else { 0 }
}

fn main() {
    let inp = readv::<i64>();
    let (a, m, l, r) = (inp[0], inp[1], inp[2], inp[3]);

    // a + k * m >= l
    // k >= ceil((l - a) / m)

    // a + k * m <= r
    // k <= floor((r - a) / m)

    let lb = ceil_div(l - a, m);
    let ub = floor_div(r - a, m);
    println!("{}", (ub - lb + 1).max(0));
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
