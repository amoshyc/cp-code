#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let inp = readv::<i64>();
    let mut dp = HashMap::new();
    let ans = f(inp[0], inp[1], inp[2], inp[3], &mut dp);
    println!("{:.7}", ans);
}

fn f(n: i64, a: i64, x: i64, y: i64, dp: &mut HashMap<i64, f64>) -> f64 {
    if n == 0 {
        return 0.0;
    }

    if dp.contains_key(&n) {
        return dp[&n];
    }

    // case 1
    let cost1 = x as f64 + f(n / a, a, x, y, dp);

    // case 2
    // f(n) = sum(1/6 * (y + f(n / b)) for b in 1..=6)
    //       = 1/6 * (y + f(n)) + sum(1/6 * (y + f(n/b)) for b in 2..=6)
    //       = 1/6 y + 1/6 f(n) + sum(1/6 * (y + f(n/b)) for b in 2..=6)
    // 5/6 f(n) = 1/6y + sum(1/6 * (y + f(n/b)) for b in 2..=6)
    // f(n) = y/5 + sum(1/5 * (y + f(n/b)) for b in 2..=6)

    let mut cost2 = y as f64 / 5.0;
    for b in 2..=6 {
        cost2 += 1.0 / 5.0 * (y as f64 + f(n / b, a, x, y, dp));
    }

    let cost = cost1.min(cost2);
    dp.insert(n, cost);

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
    read::<String>().chars().collect::<_>()
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
