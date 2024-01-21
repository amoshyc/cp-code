#![allow(unused)]

const MOD: u64 = 998_244_353;

fn main() {
    let n = read::<usize>();
    let mut cmds = vec![];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        cmds.push((inp[0] - 1, inp[1] - 1));
    }

    let mut dsu = DSU::new(2 * n);
    for i in n..(2 * n) {
        dsu.siz[i] = 0;
    }

    let mut logs = vec![];
    for (i, &(p, q)) in cmds.iter().enumerate() {
        let u = n + i;
        let (p, q) = (dsu.root(p), dsu.root(q));
        let (a, b) = (dsu.size(p) as u64, dsu.size(q) as u64);
        logs.push((u, p, q, a, b));
        dsu.unite(p, u, false);
        dsu.unite(q, u, false);
    }

    let mut ans = vec![0; 2 * n];
    for &(u, p, q, a, b) in logs.iter().rev() {
        ans[p] += ans[u];
        ans[q] += ans[u];
        ans[p] += a * powmod(a + b, MOD - 2, MOD);
        ans[q] += b * powmod(a + b, MOD - 2, MOD);
        ans[p] %= MOD;
        ans[q] %= MOD;
    }

    println!("{}", join(&ans[..n], " "));
}

fn powmod(a: u64, mut b: u64, m: u64) -> u64 {
    let mut res: u64 = 1;
    let mut base = a;
    while b > 0 {
        if b & 1 > 0 {
            res *= base;
            res %= m;
        }
        base *= base;
        base %= m;
        b >>= 1;
    }
    res
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

    fn unite(&mut self, mut u: usize, mut v: usize, can_swap: bool) {
        u = self.root(u);
        v = self.root(v);
        if u == v {
            return;
        }
        if can_swap && self.siz[u] > self.siz[v] {
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

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
