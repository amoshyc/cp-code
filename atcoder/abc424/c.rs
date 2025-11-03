#![allow(unused)]

use std::collections::VecDeque;

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut adj = vec![vec![]; n];
    let mut vis = vec![false; n];
    let mut que = VecDeque::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        if (a, b) == (0, 0) {
            vis[i] = true;
            que.push_back(i);
        } else {
            adj[a - 1].push(i);
            adj[b - 1].push(i);
        }
    }

    while let Some(u) = que.pop_front() {
        for &v in &adj[u] {
            if !vis[v] {
                vis[v] = true;
                que.push_back(v);
            }
        }
    }

    let ans = vis.iter().filter(|&x| *x).count();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
