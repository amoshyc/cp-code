#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    let mut deg = vec![0 as i32; n];
    let mut total = 0;
    for _ in 0..(n - 1) {
        let edge = readv::<i64>();
        let u = edge[0] as usize - 1;
        let v = edge[1] as usize - 1;
        let c = edge[2];
        adj[u].push((v, c));
        adj[v].push((u, c));
        total += c;
    }

    let inf = 10i64.pow(18);
    let (dis_0, _) = bfs(&adj, 0, inf);
    let s = (0..n).max_by_key(|u| dis_0[*u]).unwrap();
    let (dis_s, _) = bfs(&adj, s, inf);
    let t = (0..n).max_by_key(|u| dis_s[*u]).unwrap();

    println!("{}", 2 * total - dis_s[t]);
}

fn bfs<T>(adj: &Vec<Vec<(usize, T)>>, s: usize, inf: T) -> (Vec<T>, Vec<usize>)
where
    T: std::ops::Add<Output = T> + Ord + Default + Copy,
{
    let n = adj.len();
    let mut que = VecDeque::new();
    let mut dis = vec![inf; n];
    let mut par = vec![!0; n];
    dis[s] = T::default();
    par[s] = s;
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &(v, w) in adj[u].iter() {
            if dis[v] == inf {
                dis[v] = dis[u] + w;
                par[v] = u;
                que.push_back(v);
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
