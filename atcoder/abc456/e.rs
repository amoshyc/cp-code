#![allow(unused)]

use std::collections::{HashMap, HashSet, VecDeque};

use proconio::{input, marker::Chars, marker::Usize1};
fn main() {
    input! {
        tc: usize,
    }

    let mut ans = vec![];
    for _ in 0..tc {
        input! {
            n: usize,
            m: usize,
            es: [(Usize1, Usize1); m],
            w: usize,
            s: [Chars; n],
        }

        // Can we find a cycle in the following direct graph?

        let mut adj = vec![vec![]; n * w];
        // edges to next city
        for &(u, v) in &es {
            for i in 0..w {
                if s[u][i] == 'o' && s[v][(i + 1) % w] == 'o' {
                    let a = u * w + i;
                    let b = v * w + (i + 1) % w;
                    adj[a].push(b);
                }
                if s[v][i] == 'o' && s[u][(i + 1) % w] == 'o' {
                    let a = v * w + i;
                    let b = u * w + (i + 1) % w;
                    adj[a].push(b);
                }
            }
        }
        // don't move
        for u in 0..n {
            for i in 0..w {
                if s[u][i] == 'o' && s[u][(i + 1) % w] == 'o' {
                    let a = u * w + i;
                    let b = u * w + (i + 1) % w;
                    adj[a].push(b);
                }
            }
        }

        // cycle detection using SCC
        let (num_scc, belong) = TarjanSCC::from_adj(&adj);
        let mut sccs = vec![vec![]; num_scc];
        for u in 0..(n * w) {
            sccs[belong[u]].push(u);
        }
        if sccs.iter().any(|scc| scc.len() >= 2) {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
