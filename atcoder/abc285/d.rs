#![allow(unused)]

fn main() {
    let m = read::<usize>();
    let mut name2id = std::collections::HashMap::new();
    let mut edges = vec![];
    for _ in 0..m {
        let inp = readv::<String>();
        let size = name2id.len();
        let s = *name2id.entry(inp[0].clone()).or_insert(size);
        let size = name2id.len();
        let t = *name2id.entry(inp[1].clone()).or_insert(size);
        edges.push((s, t));
    }

    let n = name2id.len();
    let mut dsu = UnionFind::new(n);
    for (s, t) in edges {
        if dsu.same(s, t) {
            println!("No");
            return;
        } else {
            dsu.unite(s, t);
        }
    }

    println!("Yes");
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
