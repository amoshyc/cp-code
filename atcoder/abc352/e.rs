#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut ops = vec![];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (k, c) = (inp[0], inp[1] as i64);
        let verts = readv::<usize>();
        let verts = mapv(&verts, |&v| v - 1);
        ops.push((c, k, verts));
    }
    ops.sort();

    let mut mst = 0;
    let mut dsu = DSU::new(n);
    for (c, k, verts) in ops {
        let mut roots = HashSet::new();
        for u in verts {
            roots.insert(dsu.root(u));
        }

        let roots = Vec::<usize>::from_iter(roots.iter().cloned());
        for w in roots.windows(2) {
            let (u, v) = (w[0], w[1]);
            dsu.unite(u, v);
            mst += c;
        }
    }

    if (0..n).all(|u| dsu.root(u) == dsu.root(0)) {
        println!("{}", mst);
    } else {
        println!("-1");
    }
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
