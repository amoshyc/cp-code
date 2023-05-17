#![allow(unused)]

use std::collections::{VecDeque, HashSet};

fn solve() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let color = readv::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut ans = std::usize::MAX;
    let mut que = VecDeque::new();
    let mut vis = HashSet::new();
    que.push_back((0, n - 1, 0)); // (Takahashi, Aoki)
    vis.insert((0, n - 1));
    while let Some((u0, u1, d)) = que.pop_front() {
        if u0 == n - 1 && u1 == 0 {
            ans = d;
            break;
        }
        for &v0 in adj[u0].iter() {
            for &v1 in adj[u1].iter() {
                if color[v0] != color[v1] && !vis.contains(&(v0, v1)) {
                    que.push_back((v0, v1, d + 1));
                    vis.insert((v0, v1));
                }
            }
        }
    }

    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}


fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        solve();
    }
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
