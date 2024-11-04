#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut edges = vec![];
    for _ in 0..(n - 1) {
        let edge = readv::<i64>();
        let u = edge[0] as usize - 1;
        let v = edge[1] as usize - 1;
        let w = edge[2];
        edges.push((u, v, w));
    }

    edges.sort_by_key(|&(u, v, w)| w);
    let mut dsu = DSU::new(n);
    let mut ans = 0 as i64;
    for (u, v, w) in edges {
        ans += dsu.size(u) as i64 * dsu.size(v) as i64 * w;
        dsu.unite(u, v);
    }
    println!("{}", ans);
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
