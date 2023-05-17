#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<u64>();

    let mut lb = 0;
    let mut ub = arr.iter().sum::<u64>() + 1;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        let cnt = arr.iter().fold(0, |acc, &x| acc + std::cmp::min(x, m));
        if cnt / m >= (k as u64) {
            lb = m;
        } else {
            ub = m;
        }
    }

    println!("{}", lb);
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
