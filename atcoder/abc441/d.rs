#![allow(unused)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        s: i64,
        t: i64,
        edges: [(Usize1, Usize1, i64); m],
    }

    let mut adj = vec![vec![]; n];
    for (u, v, c) in edges {
        adj[u].push((v, c));
    }

    let mut ans = vec![];
    dfs(0, 0, 0, l, s, t, &adj, &mut ans);

    ans.sort();
    ans.dedup();
    println!("{}", join(&ans, " "));
}

fn dfs(
    u: usize,
    cnt: usize,
    sum: i64,
    l: usize,
    s: i64,
    t: i64,
    adj: &Vec<Vec<(usize, i64)>>,
    ans: &mut Vec<usize>,
) {
    if cnt == l {
        if s <= sum && sum <= t {
            ans.push(u + 1);
        }
        return;
    }

    for &(v, c) in &adj[u] {
        dfs(v, cnt + 1, sum + c, l, s, t, adj, ans);
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
