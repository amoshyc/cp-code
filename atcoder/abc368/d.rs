#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    let mut deg = vec![0; n];
    for i in 0..(n - 1) {
        let uv = readv::<usize>();
        let (u, v) = (uv[0] - 1, uv[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
        deg[u] += 1;
        deg[v] += 1;
    }
    let mut mark = vec![false; n];
    for u in readv::<usize>() {
        mark[u - 1] = true;
    }

    let mut que = VecDeque::new();
    let mut rem = vec![true; n];
    for u in 0..n {
        if deg[u] == 1 {
            que.push_back(u);
            deg[u] = 0;
        }
    }
    while let Some(u) = que.pop_front() {
        if mark[u] {
            continue;
        } else {
            rem[u] = false;
        }
        for &v in adj[u].iter() {
            deg[v] -= 1;
            if deg[v] == 1 {
                que.push_back(v);
            }
        }
    }
    let ans = rem.iter().filter(|u| **u).count();
    println!("{}", ans);
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
