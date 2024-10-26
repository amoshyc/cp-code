#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let uv = readv::<usize>();
        let (u, v) = (uv[0] - 1, uv[1] - 1);
        adj[u].push(v);
    }

    let inf = usize::MAX;
    let mut dis = vec![inf; adj.len()];
    let mut que = VecDeque::new();
    dis[0] = 0;
    que.push_back(0);
    while let Some(u) = que.pop_front() {
        for v in adj[u].iter() {
            if dis[*v] == inf {
                dis[*v] = dis[u] + 1;
                que.push_back(*v);
            }
        }
    }

    let mut ans = inf;
    for u in 0..n {
        if dis[u] != inf {
            if adj[u].contains(&0) {
                ans = ans.min(dis[u] + 1);
            }
        }
    }

    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans);
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
