#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let n = read::<usize>();
    let nxt = mapv(&readv::<usize>(), |&x| x - 1);

    let mut rev = vec![vec![]; n];
    for u in 0..n {
        rev[nxt[u]].push(u);
    }

    let mut cnt = vec![-1; n];
    for cycle in find_cycles_in_functional_graph(&nxt) {
        let mut que = VecDeque::new();
        for &u in cycle.iter() {
            cnt[u] = cycle.len() as i64;
            que.push_back(u);
        }
        while let Some(u) = que.pop_front() {
            for &v in rev[u].iter() {
                if cnt[v] == -1 {
                    cnt[v] = cnt[u] + 1;
                    que.push_back(v);
                }
            }
        }
    }

    println!("{}", cnt.iter().sum::<i64>());
}

fn find_cycles_in_functional_graph(nxt: &Vec<usize>) -> Vec<Vec<usize>> {
    let n = nxt.len();
    let mut dsu = DSU::new(n);
    let mut cycles = vec![];
    for u in 0..n {
        // (u, nxt[u]) is the last edge of the cycle
        if dsu.same(u, nxt[u]) {
            let mut cycle = vec![u];
            let mut x = nxt[u];
            while x != u {
                cycle.push(x);
                x = nxt[x];
            }
            cycles.push(cycle);
        } else {
            dsu.unite(u, nxt[u]);
        }
    }
    cycles
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
