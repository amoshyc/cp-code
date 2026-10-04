#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        arr: [(usize, Usize1, Usize1); q],
    }

    // ans[u] < ans[v] implies ans[u] <= ans[v]
    // Therefore we can construct graph with all <= edges.
    // A SCC in the graph need to have same value.

    let mut adj = vec![vec![]; n];
    for &(t, u, v) in &arr {
        adj[u].push(v);
    }

    // Tarjan outputs the scc in reversed topological sort.
    let (num_scc, belong) = TarjanSCC::from_adj(&adj);
    let ans = belong.iter().map(|&x| num_scc - x).collect::<Vec<_>>();

    let mut ok = true;
    for &(t, u, v) in &arr {
        if t == 0 {
            ok &= ans[u] <= ans[v];
        } else {
            ok &= ans[u] < ans[v];
        }
    }

    if ok {
        println!("Yes");
        println!("{}", join(&ans, " "));
    } else {
        println!("No");
    }
}

struct TarjanSCC {
    order: usize,
    index: Vec<usize>,
    lowlink: Vec<usize>,
    stack: Vec<usize>,
    onstack: Vec<bool>,
    scc_id: usize,
    belong: Vec<usize>,
}

impl TarjanSCC {
    fn dfs(&mut self, u: usize, adj: &Vec<Vec<usize>>) {
        self.index[u] = self.order;
        self.lowlink[u] = self.order;
        self.order += 1;
        self.stack.push(u);
        self.onstack[u] = true;

        for &v in &adj[u] {
            if self.index[v] == !0 {
                self.dfs(v, adj);
                self.lowlink[u] = self.lowlink[u].min(self.lowlink[v]);
            } else if self.onstack[v] {
                self.lowlink[u] = self.lowlink[u].min(self.index[v]);
            }
        }

        if self.index[u] == self.lowlink[u] {
            while let Some(v) = self.stack.pop() {
                self.belong[v] = self.scc_id;
                self.onstack[v] = false;
                if v == u {
                    break;
                }
            }
            self.scc_id += 1;
        }
    }

    fn from_adj(adj: &Vec<Vec<usize>>) -> (usize, Vec<usize>) {
        let n = adj.len();
        let mut data = TarjanSCC {
            order: 0,
            index: vec![!0; n],
            lowlink: vec![!0; n],
            stack: vec![],
            onstack: vec![false; n],
            scc_id: 0,
            belong: vec![!0; n],
        };
        for u in 0..n {
            if data.index[u] == !0 {
                data.dfs(u, &adj);
            }
        }
        (data.scc_id, data.belong)
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
