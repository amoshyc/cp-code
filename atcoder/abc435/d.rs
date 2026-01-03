#![allow(unused)]

use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1, Usize1); m],
        q: usize,
        asks: [(usize, Usize1); q],
    }

    let mut rev = vec![vec![]; n];
    for (u, v) in edges {
        rev[v].push(u);
    }

    let mut isblack = vec![false; n];
    let mut ans = vec![];
    for (cmd, u) in asks {
        if cmd == 1 {
            if isblack[u] {
                continue;
            }
            let mut que = VecDeque::new();
            que.push_back(u);
            isblack[u] = true;
            while let Some(u) = que.pop_front() {
                for &v in &rev[u] {
                    if !isblack[v] {
                        isblack[v] = true;
                        que.push_back(v);
                    }
                }
            }
        } else {
            if isblack[u] {
                ans.push("Yes")
            } else {
                ans.push("No");
            }
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
