#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, q) = (inp[0], inp[1], inp[2]);
    let mut edges = vec![];
    for _ in 0..m {
        let inp = readv::<i64>();
        let (u, v, w) = (inp[0] as usize - 1, inp[1] as usize - 1, inp[2]);
        edges.push((u, v, w, q));
    }
    for qid in 0..q {
        let inp = readv::<i64>();
        let (u, v, w) = (inp[0] as usize - 1, inp[1] as usize - 1, inp[2]);
        edges.push((u, v, w, qid));
    }

    let mut ans = vec![""; q];
    let mut dsu = DSU::new(n);
    edges.sort_by_key(|(u, v, w, _)| *w);
    for &(u, v, w, qid) in edges.iter() {
        if qid == q {
            if dsu.same(u, v) {
                continue;
            } else {
                dsu.unite(u, v);
            }
        } else {
            if dsu.same(u, v) {
                ans[qid] = "No";
            } else {
                ans[qid] = "Yes";
            }
        }
    }

    println!("{}", join(&ans, "\n"));
}

struct DSU {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            siz: vec![0; n],
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

    fn unite(&mut self, mut a: usize, mut b: usize) {
        a = self.root(a);
        b = self.root(b);
        if a == b {
            return;
        }
        if self.siz[a] < self.siz[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.par[a] = b;
        self.siz[b] += self.siz[a];
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
