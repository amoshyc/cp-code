#![allow(unused)]

use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];
    for &(a, b) in &ab {
        adj[a].push(b);
    }

    let mut vis = vec![false; n];
    let mut que = VecDeque::new();

    vis[0] = true;
    que.push_back(0);

    while let Some(u) = que.pop_front() {
        for &v in &adj[u] {
            if !vis[v] {
                vis[v] = true;
                que.push_back(v);
            }
        }
    }

    let ans = vis.iter().filter(|&&x| x).count();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
