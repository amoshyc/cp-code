#![allow(unused)]

use std::cmp::Reverse;

const INF: i64 = 1_000_000_000_000_000;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    let mut inv = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<i64>();
        let u = inp[0] as usize - 1;
        let v = inp[1] as usize - 1;
        let w = inp[2];
        adj[u].push((v, w));
        inv[v].push((u, w));
    }

    let mut res = vec![];
    for u in 0..n {
        let mut ans = INF;
        let dis = dijkstra(&adj, u);
        for &(v, w) in inv[u].iter() {
            ans = ans.min(w + dis[v]);
        }
        if ans == INF {
            res.push(-1);
        } else {
            res.push(ans);
        }
    }

    println!("{}", join(&res, "\n"));
}

fn dijkstra(adj: &Vec<Vec<(usize, i64)>>, root: usize) -> Vec<i64> {
    let n = adj.len();
    let mut dis = vec![INF; n];

    let mut que = std::collections::BinaryHeap::new();
    dis[root] = 0;
    que.push((Reverse(0), root));

    while let Some((Reverse(d), u)) = que.pop() {
        if d > dis[u] {
            continue;
        }
        for &(v, w) in adj[u].iter() {
            if dis[u] + w < dis[v] {
                dis[v] = dis[u] + w;
                que.push((Reverse(dis[v]), v));
            }
        }
    }

    dis
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
