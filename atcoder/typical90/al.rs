#![allow(unused)]

// [Problem]
// You are given positive integers A and B. Please find their least common multiple (LCM).
// However, if the answer exceeds 10^18, just output 10^18

// [Solution]
// lcm(a, b) = a / gcd(a, b) * b

fn main() {
    let inp = readv::<i64>();
    let (a, b) = (inp[0], inp[1]);
    let g = gcd(a, b);
    let lcm = (a / g).saturating_mul(b);
    if lcm > 10i64.pow(18) {
        println!("Large");
    } else {
        println!("{lcm}");
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
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
