#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, d) = (inp[0] as usize, inp[1]);
    let mut pos = vec![];
    for i in 0..n {
        let inp = readv::<i64>();
        pos.push((inp[0], inp[1]));
    }

    let mut dsu = UnionFind::new(n);
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = pos[i];
            let (x2, y2) = pos[j];
            if (x1 - x2).pow(2) + (y1 - y2).pow(2) <= d * d {
                dsu.unite(i, j);
            }
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        if dsu.same(0, i) {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }
    println!("{}", join(&ans, "\n"));
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
