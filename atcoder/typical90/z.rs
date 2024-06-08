#![allow(unused)]

// [Problem]
// A tree with N vertices is given. Each vertex is numbered from 1 to N, and the i-th edge (1<=i<=N-1) connects vertices Ai and Bi.
// From this tree, extract N/2 vertices such that no 2 vertices are adjacent.

// [Solution]
// We can construct an answer from a 2-coloring solution (which can be found using BFS).
// Given S1, S2 that has different color, we select the one that is larger.
// And then remove some vertices such that the size is N/2.

use std::collections::VecDeque;

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let edge = readv::<usize>();
        let u = edge[0] - 1;
        let v = edge[1] - 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut color = vec![!0; n];
    let mut que = VecDeque::new();
    color[0] = 0;
    que.push_back(0);
    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if color[v] == !0 {
                color[v] = 1 - color[u];
                que.push_back(v);
            }
        }
    }

    let mut set1: Vec<usize> = (1..=n).filter(|u| color[*u - 1] == 0).collect();
    let mut set2: Vec<usize> = (1..=n).filter(|u| color[*u - 1] == 1).collect();
    if set1.len() < set2.len() {
        std::mem::swap(&mut set1, &mut set2);
    }
    set1.truncate(n / 2);

    println!("{}", join(&set1, " "));
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
