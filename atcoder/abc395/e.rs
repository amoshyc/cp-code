#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, x) = (inp[0], inp[1], inp[2] as i64);
    let mut adj = vec![vec![]; 2 * n];
    for u in 0..n {
        let u = 2 * u;
        adj[u].push((u + 1, x));
        adj[u + 1].push((u, x));
    }
    for _ in 0..m {
        let inp = readv::<usize>();
        let u = 2 * (inp[0] - 1);
        let v = 2 * (inp[1] - 1);
        adj[u].push((v, 1));
        adj[v + 1].push((u + 1, 1));
    }

    let (dis, _) = dijkstra(&adj, 0, 10i64.pow(18));
    let ans = dis[2 * n - 1].min(dis[2 * n - 2]);
    println!("{ans}");
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
