#![allow(unused)]

use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    }

    // Consider pairs (s[i] MOD M, s[i - 1] MOD M),
    // there are only M*M possible states and form a directed graph.

    // The problem asks for the number of states that cannot reach (*, 0) or (0, *).
    // Therefore, we build a reversed graph and BFS from (*, 0) and (0, *).

    let id = |a, b| a * m + b;

    let mut rev = vec![vec![]; m * m];
    for x in 0..m {
        for y in 0..m {
            let z = (a * y + b * x) % m;
            rev[id(y, z)].push(id(x, y));
        }
    }

    let mut que = VecDeque::new();
    let mut vis = vec![false; m * m];
    for i in 0..m {
        que.push_back(id(i, 0));
        que.push_back(id(0, i));
        vis[id(i, 0)] = true;
        vis[id(0, i)] = true;
    }
    while let Some(u) = que.pop_front() {
        for &v in &rev[u] {
            if !vis[v] {
                vis[v] = true;
                que.push_back(v);
            }
        }
    }

    let ans = vis.iter().filter(|&&x| !x).count();
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
