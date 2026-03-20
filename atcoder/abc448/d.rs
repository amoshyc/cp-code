#![allow(unused)]

use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        arr: [usize; n],
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut adj = vec![vec![]; n];
    for &(u, v) in &edges {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut res = vec![false; n];
    let mut cnt = HashMap::new();
    cnt.insert(arr[0], 1);
    dfs(0, usize::MAX, &arr, &adj, &mut cnt, &mut res);

    let ans = res
        .iter()
        .map(|&x| if x { "Yes" } else { "No" })
        .collect::<Vec<_>>();

    println!("{}", join(&ans, "\n"));
}

fn dfs(
    u: usize,
    p: usize,
    arr: &Vec<usize>,
    adj: &Vec<Vec<usize>>,
    cnt: &mut HashMap<usize, usize>,
    res: &mut Vec<bool>,
) {
    for &v in &adj[u] {
        if v != p {
            if res[u] || *cnt.get(&arr[v]).unwrap_or(&0) >= 1 {
                res[v] = true;
            }

            *cnt.entry(arr[v]).or_insert(0) += 1;
            dfs(v, u, arr, adj, cnt, res);
            *cnt.entry(arr[v]).or_insert(0) -= 1;
        }
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
