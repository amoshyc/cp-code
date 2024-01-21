#![allow(unused)]

fn main() {
    let inp = readv::<u64>();
    let (n, a, b) = (inp[0], inp[1], inp[2]);

    let sum_n = (1 + n) * n / 2;

    let q = n / a;
    let sum_a = (a + a * q) * q / 2;
    
    let q = n / b;
    let sum_b = (b + b * q) * q / 2;

    let lcm = lcm(a, b);
    let q = n / lcm;
    let sum_lcm = (lcm + lcm * q) * q / 2;

    let ans = sum_n + sum_lcm - sum_a - sum_b;

    println!("{}", ans);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
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
