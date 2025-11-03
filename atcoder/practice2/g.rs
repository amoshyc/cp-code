#![allow(unused)]

use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut adj = vec![vec![]; n];
    for &(u, v) in &edges {
        adj[u].push(v);
    }

    let (num_scc, belong) = TarjanSCC::from_adj(&adj);
    let mut sccs = vec![vec![]; num_scc];
    for u in 0..n {
        sccs[belong[u]].push(u);
    }

    let mut ans = vec![format!("{}", num_scc)];
    for scc in sccs.iter().rev() {
        ans.push(format!("{} {}", scc.len(), join(&scc, " ")));
    }
    println!("{}", join(&ans, "\n"));
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
    read::<String>().chars().collect::<_>()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
