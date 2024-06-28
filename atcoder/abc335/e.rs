#![allow(unused)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let arr = readv::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let edge = readv::<usize>();
        let (u, v) = (edge[0] - 1, edge[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut dis = vec![0; n];
    let mut que = BinaryHeap::new();

    dis[0] = 1;
    que.push((Reverse(arr[0]), dis[0], 0));

    while let Some((_, d, u)) = que.pop() {
        if d < dis[u] {
            continue;
        }
        for &v in adj[u].iter() {
            let new_d = if arr[u] < arr[v] {
                dis[u] + 1
            } else if arr[u] == arr[v] {
                dis[u]
            } else {
                0
            };
            if new_d > dis[v] {
                dis[v] = new_d;
                que.push((Reverse(arr[v]), dis[v], v));
            }
        }
    }

    println!("{}", dis[n - 1]);
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
