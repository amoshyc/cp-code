#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut dsu = DSU::new(n);
    let mut extra = vec![];
    for eid in 0..m {
        let uv = readv::<usize>();
        let (u, v) = (uv[0] - 1, uv[1] - 1);
        if dsu.same(u, v) {
            extra.push((u, v, eid));
        } else {
            dsu.unite(u, v);
        }
    }

    let mut group_by_gid = HashMap::new();
    for u in 0..n {
        group_by_gid.entry(dsu.root(u)).or_insert(vec![]).push(u);
    }

    if group_by_gid.len() == 1 {
        println!("0");
        return;
    }

    let mut extra_by_gid = HashMap::new();
    for &(u, v, i) in extra.iter() {
        extra_by_gid
            .entry(dsu.root(u))
            .or_insert(vec![])
            .push((u, v, i));
    }

    let mut gids: Vec<usize> = group_by_gid.keys().cloned().collect();
    gids.sort_by_key(|gid| if extra_by_gid.contains_key(gid) { 0 } else { 1 });

    let mut ans = vec![];
    let mut extra = extra_by_gid[&gids[0]].clone();
    let anchor = group_by_gid[&gids[0]][0];

    for gid in gids.iter().skip(1) {
        if let Some(group_extra) = extra_by_gid.get(&gid) {
            // this group has extra edges
            let (u, v, eid) = group_extra[0];
            ans.push(format!("{} {} {}", eid + 1, u + 1, anchor + 1));
            extra.extend(group_extra[1..].iter());
        } else {
            // this group does not have extra edges
            let group_vert = group_by_gid[&gid][0];
            let (u, v, eid) = extra.pop().unwrap();
            ans.push(format!("{} {} {}", eid + 1, u + 1, group_vert + 1));
        }
    }

    println!("{}", ans.len());
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
