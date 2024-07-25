#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = readv::<i64>();
    let mut adj = vec![vec![]; 2 * n];
    for _ in 0..m {
        let edge = readv::<i64>();
        let u = edge[0] as usize - 1;
        let v = edge[1] as usize - 1;
        let w = edge[2];
        adj[2 * u + 1].push((2 * v, w));
        adj[2 * v + 1].push((2 * u, w));
    }
    for u in 0..n {
        adj[2 * u].push((2 * u + 1, arr[u]));
    }

    let inf = 10i64.pow(18);
    let (dis, par) = dijkstra(&adj, 0, inf);
    let ans: Vec<i64> = dis[1..].iter().step_by(2).cloned().collect();
    println!("{}", join(&ans[1..], " "));
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
