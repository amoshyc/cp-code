#![allow(unused)]

use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let inf = 10i64.pow(18);
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<i64>();
        let u = inp[0] as usize - 1;
        let v = inp[1] as usize - 1;
        let c = inp[2];
        let d = inp[3];
        adj[u].push((v, c, d));
        adj[v].push((u, c, d));
    }

    let find_arriving_time = |t_u: i64, c: i64, d: i64| -> i64 {
        // from u to v
        // arriving time f(t_w) = t_u + t_w + c + floor(d / (t_u + t_w + 1))
        // where t_w is waiting time.
        // minimum of f(t_w) occurrs when t_w is around sqrt(d) - 1 - t_u
        let mut min = t_u + c + (d / (t_u + 1)); // f(0)
        let rhs = (d as f64).sqrt() as i64 - 1 - t_u;
        for t_w in (rhs - 3)..=(rhs + 3) {
            if t_w >= 0 {
                min = min.min(t_u + t_w + c + (d / (t_u + t_w + 1)));
            }
        }
        min
    };

    let mut que = BinaryHeap::new();
    let mut dis = vec![inf; n];

    dis[0] = 0;
    que.push((Reverse(dis[0]), 0));

    while let Some((Reverse(d), u)) = que.pop() {
        if d > dis[u] {
            continue;
        }
        for &(v, c, d) in adj[u].iter() {
            let new_d = find_arriving_time(dis[u], c, d);
            if new_d < dis[v] {
                dis[v] = new_d;
                que.push((Reverse(dis[v]), v));
            }
        }
    }

    if dis[n - 1] == inf {
        println!("-1");
    } else {
        println!("{}", dis[n - 1]);
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
