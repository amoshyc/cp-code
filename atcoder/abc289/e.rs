#![allow(unused)]

use std::collections::{HashSet, VecDeque};

fn main() {
    let tc = read::<usize>();
    let mut output = vec![];
    for _ in 0..tc {
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

        let inf = std::i32::MAX;
        let mut ans = -1;
        let mut que = VecDeque::new();
        let mut dis = vec![vec![inf; n]; n];
        que.push_back((0, n - 1)); // (Takahashi, Aoki)
        dis[0][n - 1] = 0;

        while let Some((u0, u1)) = que.pop_front() {
            if u0 == n - 1 && u1 == 0 {
                break;
            }
            for &v0 in adj[u0].iter() {
                for &v1 in adj[u1].iter() {
                    if color[v0] != color[v1] && dis[u0][u1] + 1 < dis[v0][v1] {
                        que.push_back((v0, v1));
                        dis[v0][v1] = dis[u0][u1] + 1;
                    }
                }
            }
        }

        if dis[n - 1][0] == inf {
            output.push(-1);
        } else {
            output.push(dis[n - 1][0]);
        }
    }

    println!("{}", join(&output, "\n"));
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
