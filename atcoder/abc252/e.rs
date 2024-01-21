#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    // d[u] <= shortest distance[u]
    // Can we construct a spanning tree that d[u] = shortest distance[u]
    // Since shortest(a ~> b ~> c) = shortest(a ~> b) and shortest(b ~> c)
    // The last edge of the shortest path of each edge form a spanning tree!

    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for eid in 1..=m {
        let inp = readv::<i64>();
        let (u, v, w) = ((inp[0] - 1) as usize, (inp[1] - 1) as usize, inp[2]);
        adj[u].push((v, w, eid));
        adj[v].push((u, w, eid));
    }

    let inf = 10i64.pow(15);
    let mut dis = vec![inf; n];
    let mut last = vec![!0; n];
    let mut que = std::collections::BinaryHeap::new();

    dis[0] = 0;
    que.push((Reverse(dis[0]), 0));

    while let Some((Reverse(d), u)) = que.pop() {
        if d > dis[u] {
            continue;
        }
        for &(v, w, eid) in adj[u].iter() {
            let new_d = dis[u] + w;
            if new_d < dis[v] {
                dis[v] = new_d;
                last[v] = eid;
                que.push((Reverse(dis[v]), v));
            }
        }
    }

    println!("{}", join(&last[1..], " "));
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
