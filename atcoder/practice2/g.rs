#![allow(unused)]

use std::collections::BTreeSet;

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

    let (num_scc, belong) = tarjan_scc(&adj);
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
    onstack: Vec<bool>,
    stack: Vec<usize>,
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

        if self.lowlink[u] == self.index[u] {
            while let Some(x) = self.stack.pop() {
                self.onstack[x] = false;
                self.belong[x] = self.scc_id;
                if x == u {
                    break;
                }
            }
            self.scc_id += 1;
        }
    }
}

// https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm
// Returns:
//    num_scc: number of scc
//    belong: which scc each vertex belongs to
// The order of scc is a *reversed* topological sort of the DAG.
fn tarjan_scc(adj: &Vec<Vec<usize>>) -> (usize, Vec<usize>) {
    let n = adj.len();
    let mut data = TarjanSCC {
        order: 0,
        index: vec![!0; n],
        lowlink: vec![0; n],
        onstack: vec![false; n],
        stack: vec![],
        scc_id: 0,
        belong: vec![!0; n],
    };
    for root in 0..n {
        if data.index[root] == !0 {
            data.dfs(root, adj);
        }
    }
    (data.scc_id, data.belong)
}


fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
