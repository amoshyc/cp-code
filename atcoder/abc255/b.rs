#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<usize>();
    let mut pos = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        pos.push((inp[0], inp[1]));
    }

    let mut ans = 0;
    for i in 0..n {
        let mut need = 10i64.pow(18);
        for &k in arr.iter() {
            let (x1, y1) = pos[k - 1];
            let (x2, y2) = pos[i];
            let d2 = (x1 - x2).pow(2) + (y1 - y2).pow(2);
            need = need.min(d2);
        }
        ans = ans.max(need);
    }
    
    println!("{:.10}", (ans as f64).sqrt());
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
            std::mem::swap(&mut u, &mut v);
        }
        self.par[u] = v;
        self.siz[v] += self.siz[u];
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }

    fn size(&mut self, mut u: usize) -> usize {
        u = self.root(u);
        self.siz[u]
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
