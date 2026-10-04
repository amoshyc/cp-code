#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        y: i64,
        es: [(Usize1, Usize1, i64); m],
        xs: [i64; n],
    }

    // u -------> v
    //      w

    // u -----> A --> B -----> v
    //   xs[u]     y     xs[v]

    let mut adj = vec![vec![]; n + 2];

    let (a, b) = (n, n + 1);
    adj[a].push((b, y));
    for u in 0..n {
        adj[u].push((a, xs[u]));
        adj[b].push((u, xs[u]));
    }

    for &(u, v, w) in &es {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let (dis, _) = dijkstra(&adj, 0, 10i64.pow(18));
    println!("{}", join(&dis[1..n], " "));
}

use std::cmp::Reverse;

fn dijkstra<T>(adj: &Vec<Vec<(usize, T)>>, s: usize, inf: T) -> (Vec<T>, Vec<usize>)
where
    T: std::ops::Add<Output = T> + Ord + Copy + Default,
{
    let n = adj.len();
    let mut que = std::collections::BinaryHeap::new(); // max heap
    let mut dis = vec![inf; n];
    let mut par = vec![!0; n];

    dis[s] = T::default();
    par[s] = s;
    que.push((Reverse(dis[s]), s));

    while let Some((Reverse(d), u)) = que.pop() {
        if d > dis[u] {
            continue;
        }
        for &(v, w) in adj[u].iter() {
            let new_d = dis[u] + w;
            if new_d < dis[v] {
                dis[v] = new_d;
                par[v] = u;
                que.push((Reverse(dis[v]), v));
            }
        }
    }

    (dis, par)
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
