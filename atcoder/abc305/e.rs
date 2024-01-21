#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut roots = vec![];
    for _ in 0..k {
        let inp = readv::<usize>();
        let (p, h) = (inp[0] - 1, inp[1] + 1);
        roots.push((p, h));
    }


    let mut que = std::collections::BinaryHeap::new();
    let mut cap = vec![0; n];
    for &(p, h) in roots.iter() {
        que.push((h, p));
        cap[p] = h;
    }

    while let Some((h, u)) = que.pop() {
        if h < cap[u] {
            continue;
        }
        for &v in adj[u].iter() {
            let new_cap = cap[u] - 1;
            if new_cap > 0 && new_cap > cap[v] {
                cap[v] = new_cap;
                que.push((new_cap, v));
            }
        }
    }

    let mut ans = vec![];
    for u in 0..n {
        if cap[u] > 0 {
            ans.push(u + 1);
        }
    }
    println!("{}\n{}", ans.len(), join(&ans, " "));
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
