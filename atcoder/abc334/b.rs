#![allow(unused)]

// a/b rounds toward zero, therefore
// when a > 0, we right shift a,
// when a < 0, a/b is what we want
fn ceil_div(mut a: i64, mut b: i64) -> i64 {
    assert!(b != 0);
    a *= b.signum();
    b = b.abs();
    if a >= 0 {
        (a + b - 1) / b
    } else {
        a / b
    }
}

// a/b rounds toward zero, therefore
// when a > 0, a/b is what we want
// when a < 0, we left shift a.
fn floor_div(mut a: i64, mut b: i64) -> i64 {
    assert!(b != 0);
    a *= b.signum();
    b = b.abs();
    if a >= 0 {
        a / b
    } else {
        (a - (b - 1)) / b
    }
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
