#![allow(unused)]

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1, i64); m],
    }

    let edges = edges
        .into_iter()
        .map(|(u, v, w)| (u, v, -w))
        .collect::<Vec<_>>();

    let inf = 10i64.pow(18);
    let dis = bellman_ford(&edges, n, 0, inf);
    if dis[n - 1] == -inf {
        println!("inf");
    } else {
        println!("{}", -dis[n - 1]);
    }
}

fn bellman_ford(edges: &[(usize, usize, i64)], n: usize, s: usize, inf: i64) -> Vec<i64> {
    // if an vertex is relaxed >= n - 1 times, 
    // then it is on a negative cycle or can be reached from a negative cycle.
    // To find all the vertices that can be reached from a negative cycle, 
    // we simply relax the graph v more times.
    // dis[u] = inf: u is unreachable
    // dis[u] = -inf: u can reached from a negative cycle.
    let mut dis = vec![inf; n];
    dis[s] = 0;
    for r in 0..(2 * n - 1) {
        for &(u, v, w) in edges {
            if dis[v] > dis[u] + w {
                dis[v] = dis[u] + w;
                if r >= n - 1 {
                    dis[v] = -inf;
                }
            }
        }
    }
    dis
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
