#![allow(unused)]

fn main() {
    let inp = readv::<u64>();
    let (n, m, k) = (inp[0] as usize, inp[1] as usize, inp[2]);
    let mut edges = vec![];
    for _ in 0..m {
        let inp = readv::<u64>();
        let (u, v, w) = (inp[0] as usize - 1, inp[1] as usize - 1, inp[2]);
        edges.push((u, v, w));
    }

    let mut ans = k;

    let mut mask = vec![0; m];
    for i in 0..n - 1 {
        mask[m - 1 - i] = 1;
    }

    for subset in combinations(&edges, n - 1) {
        let mut dsu = DSU::new(n);
        let mut val = 0;
        for &(u, v, w) in subset.iter() {
            if !dsu.same(u, v) {
                dsu.unite(u, v);
                val = (val + w) % k;
            }
        }
        let mut cnt = 0;
        for u in 0..n {
            if dsu.root(u) == u {
                cnt += 1;
            }
        }

        if cnt == 1 {
            ans = ans.min(val);
        }
    }

    println!("{}", ans);
}

// Modified from https://docs.python.org/3/library/itertools.html#itertools.combinations
fn combinations<T: Copy>(arr: &[T], r: usize) -> impl std::iter::Iterator<Item = Vec<T>> + '_ {
    assert!(r <= arr.len());
    let n = arr.len();
    let mut indices = (0..r).collect::<Vec<usize>>();
    let iter1 = std::iter::once(indices.iter().map(|&i| arr[i]).collect::<Vec<T>>());
    let iter2 = std::iter::from_fn(move || {
        while let Some(i) = (0..r).rposition(|j| indices[j] != j + n - r) {
            indices[i] += 1;
            for j in (i + 1)..r {
                indices[j] = indices[j - 1] + 1
            }
            let comb = indices.iter().map(|&i| arr[i]).collect::<Vec<T>>();
            return Some(comb);
        }
        None
    });
    iter1.chain(iter2)
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
