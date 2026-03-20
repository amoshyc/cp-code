#![allow(unused)]

use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        xyp: [(i64, i64, i64); n],
    }

    let mut xs = vec![0; n];
    let mut ys = vec![0; n];
    let mut ps = vec![0; n];
    for i in 0..n {
        (xs[i], ys[i], ps[i]) = xyp[i];
    }

    let ok = |s: i64| -> bool {
        let mut adj = vec![vec![]; n];
        for u in 0..n {
            for v in 0..n {
                if u != v && s * ps[u] >= (xs[u] - xs[v]).abs() + (ys[u] - ys[v]).abs() {
                    adj[u].push(v);
                }
            }
        }

        let (num_scc, belong) = TarjanSCC::from_adj(&adj);
        let root = (0..n).find(|&u| belong[u] == num_scc - 1).unwrap();

        let mut vis = vec![false; n];
        let mut que = VecDeque::new();

        vis[root] = true;
        que.push_back(root);

        while let Some(u) = que.pop_front() {
            for &v in &adj[u] {
                if !vis[v] {
                    vis[v] = true;
                    que.push_back(v);
                }
            }
        }

        vis.iter().all(|&x| x)
    };

    // 0 0 0 1 1 1
    let mut lb = 0;
    let mut ub = 10i64.pow(10);
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if ok(m) {
            ub = m;
        } else {
            lb = m;
        }
    }

    println!("{ub}");
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
