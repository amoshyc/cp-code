#![allow(unused)]

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut dsu = DSU::new(n);
    let mut count = vec![0; n];
    let mut color = vec![0; n];

    let mut ans = vec![];
    for _ in 0..q {
        input! {cmd: usize}
        if cmd == 1 {
            input! {u: Usize1, v: Usize1}
            dsu.unite(u, v);
        } else if cmd == 3 {
            input! {v: Usize1}
            let r = dsu.root(v);
            if dsu.cnt[r] > 0 {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
        } else {
            input! {v: Usize1}
            color[v] ^= 1;
            let r = dsu.root(v);
            if color[v] == 1 {
                // 0 -> 1
                dsu.cnt[r] += 1;
            } else {
                // 1 -> 0
                dsu.cnt[r] -= 1;
            }
        }
    }

    println!("{}", join(&ans, "\n"));
}

struct DSU {
    par: Vec<usize>,
    siz: Vec<usize>,
    cnt: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            siz: vec![1; n],
            cnt: vec![0; n],
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
            self.cnt[u] += self.cnt[v];
        } else {
            self.par[u] = v;
            self.siz[v] += self.siz[u];
            self.cnt[v] += self.cnt[u];
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
