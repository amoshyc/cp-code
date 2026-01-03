#![allow(unused)]

use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut adj = vec![vec![]; h * w + 26];

    for r in 0..h {
        for c in 0..w {
            // walk
            if s[r][c] != '#' {
                let u = r * w + c;
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nr = r.checked_add_signed(dr).unwrap_or(h);
                    let nc = c.checked_add_signed(dc).unwrap_or(w);
                    if nr < h && nc < w && s[nr][nc] != '#' {
                        let v = nr * w + nc;
                        adj[u].push((v, 1));
                    }
                }
            }
            // warp
            if s[r][c].is_ascii_lowercase() {
                let u = r * w + c;
                let z = h * w + s[r][c] as usize - 'a' as usize;
                adj[u].push((z, 1));
                adj[z].push((u, 0));
            }
        }
    }

    let inf = 1_000_000_000;
    let (dis, _) = bfs01(&adj, 0, inf);

    let ans = dis[(h - 1) * w + (w - 1)];
    if ans != inf {
        println!("{ans}");
    } else {
        println!("-1");
    }
}

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
