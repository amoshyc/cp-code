#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let t = reads();
    let n = read::<usize>();

    // dp[i, j] = minimum cost to make t[..j] using bags[..=i]

    let inf = 10usize.pow(9);
    let mut dp = vec![vec![inf; t.len()]; n];

    // base
    let arr = readv::<String>();
    for s in arr[1..].iter() {
        let s: Vec<char> = s.chars().collect();
        if s.len() <= t.len() && s == t[..s.len()] {
            dp[0][s.len() - 1] = 1;
        }
    }

    // recursive
    for i in 1..n {
        let mut bags = vec![];
        let arr = readv::<String>();
        for s in arr[1..].iter() {
            bags.push(s.chars().collect::<Vec<char>>());
        }

        for j in 0..t.len() {
            dp[i][j] = dp[i - 1][j];
            for s in bags.iter() {
                if j == s.len() - 1 && *s == t[..s.len()] {
                    dp[i][j] = dp[i][j].min(1);
                }
                if j >= s.len() && *s == t[j + 1 - s.len()..=j] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - s.len()] + 1);
                }
            }
        }
    }

    if dp[n - 1][t.len() - 1] == inf {
        println!("-1");
    } else {
        println!("{}", dp[n - 1][t.len() - 1]);
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
