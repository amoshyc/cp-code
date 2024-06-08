#![allow(unused)]

use std::collections::{HashSet, VecDeque};

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let u = inp[0] - 1;
        let v = inp[1] - 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut ans = 0 as i64;
    let mut vis = vec![false; n];
    for root in 0..n {
        if vis[root] {
            continue;
        }

        vis[root] = true;
        let mut v_cnt = 0 as i64;
        let mut que = VecDeque::from([root]);
        let mut edges = HashSet::new();
        while let Some(u) = que.pop_front() {
            v_cnt += 1;
            for &v in adj[u].iter() {
                edges.insert((u.min(v), u.max(v)));
                if !vis[v] {
                    vis[v] = true;
                    que.push_back(v);
                }
            }
        }

        if v_cnt >= 3 {
            ans += v_cnt * (v_cnt - 1) / 2 - edges.len() as i64;
        }
    }

    println!("{}", ans);
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
    read::<String>().chars().collect::<_>()
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
