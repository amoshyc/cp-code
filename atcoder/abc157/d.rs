#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);

    let mut dsu = DSU::new(n);
    let mut adj = vec![vec![]; n];
    let mut ban = vec![vec![]; n];

    for _ in 0..m {
        let edge = readv::<usize>();
        let (u, v) = (edge[0] - 1, edge[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
        dsu.unite(u, v);
    }
    for _ in 0..k {
        let edge = readv::<usize>();
        let (u, v) = (edge[0] - 1, edge[1] - 1);
        ban[u].push(v);
        ban[v].push(u);
    }

    // A = {v for v in group(u) if (u, v) is friend}
    // B = {v for v in group(u) if (u, v) is banned}
    // ans(u) = (group size - 1) - n(A or B)
    //        = (group size - 1) - n(A) - n(B) + n(A and B)
    //        = (group size - 1) - n(A) - n(B) + 0

    let mut ans = vec![0; n];
    for u in 0..n {
        let cnt_a = adj[u].len();
        let cnt_b = ban[u].iter().filter(|v| dsu.same(u, **v)).count();
        ans[u] = dsu.size(u) - 1 - cnt_a - cnt_b;
    }

    println!("{}", join(&ans, " "));
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
