#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (n, m, q) = (inp[0], inp[1], inp[2]);

    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<i64>();
        let (u, v, w) = (inp[0] as usize - 1, inp[1] as usize - 1, inp[2]);
        adj[u].push((v, w));
        adj[v].push((u, -w));
    }

    let mut dsu = UnionFind::new(n);
    for u in 0..n {
        for &(v, w) in adj[u].iter() {
            dsu.unite(u, v);
        }
    }

    // println!("{:?}", dsu.groups());

    let mut dis = vec![std::i64::MAX; n];
    let mut cyc = vec![false; n];
    for r in 0..n {
        if dsu.root(r) == r {
            let mut que = VecDeque::new();
            dis[r] = 0;
            que.push_back(r);
            while let Some(u) = que.pop_front() {
                for &(v, w) in adj[u].iter() {
                    let new_d = dis[u].saturating_add(w);
                    if new_d < dis[v] {
                        if dis[v] != std::i64::MAX {
                            cyc[r] = true;
                        } else {
                            dis[v] = new_d;
                            que.push_back(v);
                        }
                    }
                }
            }
        }
    }

    // println!("{:?}", dis);
    // println!("{:?}", cyc);

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);

        if !dsu.same(u, v) {
            ans.push("nan".to_string());
        } else if cyc[dsu.root(u)] {
            ans.push("inf".to_string());
        } else {
            ans.push((dis[v] - dis[u]).to_string());
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

    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut groups = std::collections::HashMap::<usize, Vec<usize>>::new();
        for u in 0..self.parent.len() {
            groups.entry(self.root(u)).or_insert(vec![]).push(u);
        }
        groups.values().map(|g| g.clone()).collect()
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}