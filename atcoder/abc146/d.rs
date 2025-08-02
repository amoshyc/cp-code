#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    for eid in 0..(n - 1) {
        let uv = readv::<usize>();
        let (u, v) = (uv[0] - 1, uv[1] - 1);
        adj[u].push((v, eid));
        adj[v].push((u, eid));
    }

    let root = (0..n).max_by_key(|u| adj[*u].len()).unwrap();
    let max_deg = adj[root].len();
    let mut color = vec![usize::MAX; n - 1];
    dfs(root, usize::MAX, max_deg - 1, max_deg, &adj, &mut color);

    let ans = mapv(&color, |&x| x + 1);
    println!("{}", max_deg);
    println!("{}", join(&ans, "\n"));
}

fn dfs(
    u: usize,
    p: usize,
    mut c: usize,
    max_deg: usize,
    adj: &Vec<Vec<(usize, usize)>>,
    color: &mut Vec<usize>,
) {
    for &(v, eid) in &adj[u] {
        if v != p {
            c = (c + 1) % max_deg;
            color[eid] = c;
            dfs(v, u, c, max_deg, adj, color);
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
