#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut deg = vec![0; n];
    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        deg[u] += 1;
        deg[v] += 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut dsu = DSU::new(n);
    for u in 0..n {
        for &v in adj[u].iter() {
            if deg[u] == 3 && deg[v] == 3 {
                dsu.unite(u, v);
            }
        }
    }

    let mut groups = vec![vec![]; n];
    for u in 0..n {
        if deg[u] == 2 {
            for &v in adj[u].iter() {
                if deg[v] == 3 {
                    let r = dsu.root(v);
                    groups[r].push(u);
                }
            }
        }
    }

    let mut ans = 0;
    for group in groups {
        if group.len() >= 2 {
            let g = group.len() as i64;
            ans += g * (g - 1) / 2;
        }
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

fn reads() -> Vec<char> {
    read::<String>().chars().collect()
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
