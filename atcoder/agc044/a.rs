#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let t = read::<usize>();
    for _ in 0..t {
        let inp = readv::<i64>();
        let n = inp[0];
        let ws = inp[1..].to_vec();
        let mut dp = HashMap::new();
        println!("{}", f(n, 0, &ws, &mut dp));
    }
}

fn f(x: i64, t: i64, ws: &Vec<i64>, dp: &mut HashMap<i64, i64>) -> i64 {
    if dp.contains_key(&x) {
        return dp[&x];
    }

    if x <= t {
        return (t - x).saturating_mul(ws[3]);
    }

    let mut ans = (t - x).abs().saturating_mul(ws[3]);
    for (&k, &w) in vec![2, 3, 5].iter().zip(ws.iter()) {
        // x -> k * v -> v
        let v1 = x / k;
        let v2 = (x + k - 1) / k;
        if v1 != x {
            let c1 = (k * v1 - x).abs().saturating_mul(ws[3]) + w + f(v1, t, ws, dp);
            ans = ans.min(c1);
        }
        if v2 != x {
            let c2 = (k * v2 - x).abs().saturating_mul(ws[3]) + w + f(v2, t, ws, dp);
            ans = ans.min(c2);
        }
    }

    dp.insert(x, ans);
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
