#![allow(unused)]

use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<i64>();
        let (l, d, k, c) = (inp[0], inp[1], inp[2], inp[3]);
        let (a, b) = (inp[4] as usize - 1, inp[5] as usize - 1);
        adj[b].push((a, l, d, k, c));
    }

    let inf = 2 * 10i64.pow(18);
    let mut tim = vec![-inf; n];
    let mut que = BinaryHeap::new();
    tim[n - 1] = inf;
    que.push((tim[n - 1], n - 1));
    while let Some((t, u)) = que.pop() {
        if t < tim[u] {
            continue;
        }
        // Currently at u at time t
        for &(v, l, d, k, c) in adj[u].iter() {
            // Find the largest x (0 <= x < k) such that
            // l + x * d + c <= t
            // Then the arrival time of v is
            // l + x * d
            let x = ((t - c - l) / d).min(k - 1);
            if x < 0 {
                continue;
            }
            let new_t = l + x * d;
            if new_t > tim[v] {
                tim[v] = new_t;
                que.push((tim[v], v));
            }
        }
    }

    let ans = mapv(&tim, |&t| {
        if t == -inf {
            "Unreachable".to_string()
        } else {
            format!("{}", t)
        }
    });
    println!("{}", join(&ans[..(n - 1)], "\n"));
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
