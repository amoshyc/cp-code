#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1] as u64);
    let arr = readv::<u64>();

    let mut edges = vec![];
    for u in 0..n {
        for v in (u + 1)..n {
            let w1 = powmod(arr[u], arr[v], m);
            let w2 = powmod(arr[v], arr[u], m);
            edges.push((u, v, (w1 + w2) % m));
        }
    }

    // Kruskal's Algorithm
    edges.sort_by_key(|&(u, v, w)| std::cmp::Reverse(w));
    let mut total: u64 = 0;
    let mut dsu = UnionFind::new(n);
    for (u, v, w) in edges {
        if !dsu.same(u, v) {
            dsu.unite(u, v);
            total += w;
        }
    }
    println!("{}", total);
}

fn powmod(a: u64, mut b: u64, m: u64) -> u64 {
    let mut base = a % m;
    let mut res = 1;
    while b != 0 {
        if b & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        b >>= 1;
    }
    res
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            self.parent[x] = self.root(self.parent[x]);
            self.parent[x]
        }
    }

    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.size[x]
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    fn unite(&mut self, mut a: usize, mut b: usize) {
        a = self.root(a);
        b = self.root(b);
        if a == b {
            return;
        }
        if self.size(a) > self.size(b) {
            std::mem::swap(&mut a, &mut b);
        }
        self.parent[a] = b;
        self.size[b] += self.size[a];
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
