#![allow(unused)]

use std::collections::{HashSet, VecDeque};

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let uv = readv::<usize>();
        let (u, v) = (uv[0] - 1, uv[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut ans = vec![-1; n];
    // k = 0
    let mut neighbors = HashSet::new();
    let mut waiting = HashSet::new();
    let mut completed = HashSet::new();
    for &v in &adj[0] {
        neighbors.insert(v);
    }
    ans[0] = neighbors.len() as i32;
    completed.insert(0);

    for k in 1..n {
        if neighbors.contains(&k) {
            let mut que = VecDeque::new();
            que.push_front(k);
            while let Some(u) = que.pop_front() {
                neighbors.remove(&u);
                completed.insert(u);
                for &v in &adj[u] {
                    if !completed.contains(&v) {
                        neighbors.insert(v);
                        if waiting.contains(&v) {
                            waiting.remove(&v);
                            que.push_back(v);
                        }
                    }
                }
            }
            if waiting.len() == 0 {
                ans[k] = neighbors.len() as i32;
            }
            // println!("k={}: {:?}", k, neighbors);
        } else {
            waiting.insert(k);
            ans[k] = -1;
        }
    }

    println!("{}", join(&ans, "\n"));
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
