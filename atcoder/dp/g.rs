#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];
    for &(u, v) in &edges {
        adj[u].push(v);
    }

    let mut sort = topological_sort(&adj);

    // dp[u] = length of the longest path starting from u
    // dp[u] = max(dp[v] for v in adj[u]) + 1
    let mut dp = vec![0; n];
    for &u in sort.iter().rev() {
        for &v in &adj[u] {
            dp[u] = dp[u].max(dp[v] + 1);
        }
    }

    println!("{}", dp.iter().max().unwrap());
}

fn topological_sort(adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = adj.len();
    let mut indeg = vec![0; n];
    for u in 0..n {
        for &v in adj[u].iter() {
            indeg[v] += 1;
        }
    }

    let mut que = std::collections::VecDeque::new();
    for u in 0..n {
        if indeg[u] == 0 {
            que.push_back(u);
        }
    }

    let mut nodes = vec![];
    while let Some(u) = que.pop_front() {
        nodes.push(u);
        for &v in adj[u].iter() {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                que.push_back(v);
            }
        }
    }

    nodes
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
