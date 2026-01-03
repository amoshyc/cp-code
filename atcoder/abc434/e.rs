#![allow(unused)]

use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        arr: [(i64, i64); n],
    }

    let mut xs = vec![];
    for &(x, r) in &arr {
        xs.push(x - r);
        xs.push(x + r);
    }
    xs.sort();
    xs.dedup();

    let edges = arr
        .iter()
        .map(|&(x, r)| {
            (
                xs.binary_search(&(x - r)).unwrap(),
                xs.binary_search(&(x + r)).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    // if the CC has least one cycle, then all vertices inside can be picked.
    // Otherwise, the answer of V - 1.

    let num_vert = xs.len();
    let mut dsu = DSU::new(num_vert);
    let mut in_cycle = vec![false; num_vert];
    for &(u, v) in &edges {
        if dsu.same(u, v) {
            in_cycle[u] = true;
            in_cycle[v] = true;
        } else {
            dsu.unite(u, v);
        }
    }

    let mut ccs = HashMap::new();
    for u in 0..num_vert {
        ccs.entry(dsu.root(u)).or_insert(vec![]).push(u);
    }

    let mut ans = 0;
    for cc in ccs.values() {
        if cc.iter().any(|&u| in_cycle[u]) {
            ans += cc.len();
        } else {
            ans += cc.len() - 1;
        }
    }

    println!("{ans}");
}

struct DSU {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, u: usize) -> usize {
        if self.par[u] == u {
            u
        } else {
            self.par[u] = self.root(self.par[u]);
            self.par[u]
        }
    }

    fn unite(&mut self, mut u: usize, mut v: usize) {
        u = self.root(u);
        v = self.root(v);
        if u == v {
            return;
        }
        if self.siz[u] > self.siz[v] {
            self.par[v] = u;
            self.siz[u] += self.siz[v];
        } else {
            self.par[u] = v;
            self.siz[v] += self.siz[u];
        }
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    fn size(&mut self, u: usize) -> usize {
        let r = self.root(u);
        self.siz[r]
    }
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
