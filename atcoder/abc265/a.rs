#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (x, y, n) = (inp[0], inp[1], inp[2]);
    let case1 = n * x;
    let case2 = n / 3 * y + (n % 3) * x;
    println!("{}", std::cmp::min(case1, case2));
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