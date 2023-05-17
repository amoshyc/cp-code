#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; 2 * n];
    for i in 0..n {
        adj[2 * i + 0].push(2 * i + 1);
        adj[2 * i + 1].push(2 * i + 0);
    }
    for _ in 0..m {
        let inp = readv::<String>();
        let (a, b, c, d) = (
            inp[0].clone(),
            inp[1].clone(),
            inp[2].clone(),
            inp[3].clone(),
        );
        let a: usize = a.parse().unwrap();
        let c: usize = c.parse().unwrap();
        let u = (a - 1) * 2 + if b == "R" { 0 } else { 1 };
        let v = (c - 1) * 2 + if d == "R" { 0 } else { 1 };
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut vis = vec![false; 2 * n];
    let mut cnt1 = 0;
    let mut cnt2 = 0;
    for root in 0..2 * n {
        if vis[root] {
            continue;
        }

        let mut stack = vec![root];
        let mut verts = HashSet::new();
        verts.insert(root);
        while let Some(u) = stack.pop() {
            for &v in adj[u].iter() {
                if !vis[v] {
                    vis[v] = true;
                    stack.push(v);
                    verts.insert(v);
                }
            }
        }

        if verts.iter().all(|&v| adj[v].len() % 2 == 0) {
            cnt1 += 1;
        } else {
            cnt2 += 1;
        }
    }

    println!("{} {}", cnt1, cnt2);
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
