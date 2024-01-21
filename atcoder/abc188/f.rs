#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let inp = readv::<i64>();
    let (x, y) = (inp[0], inp[1]);
    let mut dp = HashMap::new();
    println!("{}", f(y, x, &mut dp));
}

fn f(s: i64, x: i64, dp: &mut HashMap<i64, i64>) -> i64 {
    if dp.contains_key(&s) {
        return dp[&s];
    }

    if s <= x {
        dp.insert(s, x - s);
        return x - s;
    }

    // s -> floor(s / 2)
    let v1 = s / 2;
    let c1 = (s - v1 * 2).abs() * 1 + 1 + f(v1, x, dp);
    // s -> ceil(s / 2)
    let v2 = (s + 1) / 2;
    let c2 = (s - v2 * 2).abs() * 1 + 1 + f(v2, x, dp);
    
    let mut ans = s - x;
    ans = ans.min(c1);
    ans = ans.min(c2);
    dp.insert(s, ans);
    ans
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
