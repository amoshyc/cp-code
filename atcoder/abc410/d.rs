#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let uvw = readv::<usize>();
        let (u, v, w) = (uvw[0] - 1, uvw[1] - 1, uvw[2]);
        adj[u].push((v, w));
    }

    // vis[u, x] = can I arrive at u with cumulative xor = x
    let mut vis = vec![vec![false; 1024]; n];
    let mut que = VecDeque::new();

    vis[0][0] = true;
    que.push_back((0, 0));

    while let Some((u, xor)) = que.pop_front() {
        for &(v, w) in &adj[u] {
            let new_xor = xor ^ w;
            if vis[v][new_xor] == false {
                vis[v][new_xor] = true;
                que.push_back((v, new_xor));
            }
        }
    }

    if let Some(ans) = (0..1024).find(|&x| vis[n - 1][x]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
