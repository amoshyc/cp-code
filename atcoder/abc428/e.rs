#![allow(unused)]

use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut adj = vec![vec![]; n];
    for (u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let inf = 1_000_000_000;
    let (dis_0, _) = bfs(&adj, 0, inf);
    let s = (0..n).max_by_key(|&u| dis_0[u]).unwrap();
    let (dis_s, _) = bfs(&adj, s, inf);
    let t = (0..n).max_by_key(|&u| dis_s[u]).unwrap();
    let (dis_t, _) = bfs(&adj, t, inf);

    let mut ans = vec![0; n];
    for u in 0..n {
        if dis_s[u] > dis_t[u] {
            ans[u] = s + 1;
        } else if dis_s[u] == dis_t[u] {
            ans[u] = s.max(t) + 1;
        } else {
            ans[u] = t + 1;
        }
    }
    println!("{}", join(&ans, "\n"));
}

fn bfs(adj: &Vec<Vec<usize>>, s: usize, inf: usize) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut que = VecDeque::new();
    let mut dis = vec![inf; n];
    let mut par = vec![!0; n];
    dis[s] = 0;
    par[s] = s;
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if dis[v] == inf {
                dis[v] = dis[u] + 1;
                par[v] = u;
                que.push_back(v);
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
