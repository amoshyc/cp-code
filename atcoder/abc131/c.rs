#![allow(unused)]

fn main() {
    let inp = readv::<u64>();
    let (a, b, c, d) = (inp[0], inp[1], inp[2], inp[3]);
    let lcm = c / gcd(c, d) * d;
    let ans = b - a + 1 + count(lcm, a, b) - count(c, a, b) - count(d, a, b);
    println!("{ans}");
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn count(m: u64, a: u64, b: u64) -> u64 {
    b / m - (a - 1) / m
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
