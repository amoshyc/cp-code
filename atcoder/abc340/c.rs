#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<u64>();
    let mut dp = HashMap::new();
    println!("{}", f(n, &mut dp));
}

fn f(x: u64, dp: &mut HashMap<u64, u64>) -> u64 {
    if x < 2 {
        return 0;
    }

    if dp.contains_key(&x) {
        return dp[&x];
    }

    let cost = x + f(x / 2, dp) + f((x + 1) / 2, dp);
    dp.insert(x, cost);

    cost
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
