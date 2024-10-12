#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let edge = readv::<i64>();
        let u = edge[0] as usize - 1;
        let v = edge[1] as usize - 1;
        let w = edge[2];
        adj[u].push((v, w));
        adj[v].push((u, -w));
    }

    let inf = 10i64.pow(18) + 1;
    let mut dis = vec![inf; n];
    for r in 0..n {
        if dis[r] != inf {
            continue;
        }

        let mut que = VecDeque::new();
        que.push_back(r);
        dis[r] = 0;
        while let Some(u) = que.pop_front() {
            for &(v, w) in adj[u].iter() {
                if dis[v] == inf {
                    dis[v] = dis[u] + w;
                    que.push_back(v);
                }
            }
        }
    }

    println!("{}", join(&dis, " "));
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
