#![allow(unused)]

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        tc: usize,
    }

    let dirs = ['A', 'B', 'C'];
    let valid = [
        [(1, 3), (0, 2), (2, 0), (3, 1)], // A
        [(1, 2), (0, 3), (2, 1), (3, 0)], // B
        [(1, 0), (0, 1), (2, 3), (3, 2)], // C
    ];

    let mut ans = vec![];
    for _ in 0..tc {
        input! {
            h: usize,
            w: usize,
            s: [Chars; h],
        }

        //       0
        // 1  (r, c)   3
        //       2
        let id = |r: usize, c: usize, d: usize| (r * w + c) * 4 + d;

        let mut adj = vec![vec![]; h * w * 4];
        for r in 0..h {
            for c in 0..w {
                if r >= 1 {
                    adj[id(r, c, 0)].push((id(r - 1, c, 2), 0));
                }
                if c >= 1 {
                    adj[id(r, c, 1)].push((id(r, c - 1, 3), 0));
                }
                if r + 1 < h {
                    adj[id(r, c, 2)].push((id(r + 1, c, 0), 0));
                }
                if c + 1 < w {
                    adj[id(r, c, 3)].push((id(r, c + 1, 1), 0));
                }
            }
        }

        // change (r, c) to type k
        for r in 0..h {
            for c in 0..w {
                for k in 0..3 {
                    let cost: usize = if s[r][c] == dirs[k] { 0 } else { 1 };
                    for (src, dst) in valid[k] {
                        adj[id(r, c, src)].push((id(r, c, dst), cost));
                    }
                }
            }
        }

        let (dis, _) = bfs01(&adj, id(0, 0, 1), 1_000_000);
        ans.push(dis[id(h - 1, w - 1, 3)]);
    }

    println!("{}", join(&ans, "\n"));
}

use std::collections::VecDeque;

fn bfs01<T>(adj: &Vec<Vec<(usize, T)>>, s: usize, inf: T) -> (Vec<T>, Vec<usize>)
where
    T: std::ops::Add<Output = T> + Copy + Default + Eq + Ord,
{
    let n = adj.len();
    let mut que = VecDeque::new();
    let mut dis = vec![inf; n];
    let mut par = vec![!0; n];

    dis[s] = T::default();
    par[s] = s;
    que.push_back((s, dis[s]));

    while let Some((u, d)) = que.pop_front() {
        if d > dis[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let new_d = dis[u] + w;
            if new_d < dis[v] {
                dis[v] = new_d;
                par[v] = u;
                if w == T::default() {
                    que.push_front((v, dis[v]));
                } else {
                    que.push_back((v, dis[v]));
                }
            }
        }
    }

    (dis, par)
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
