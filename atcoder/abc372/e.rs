#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);
    let mut dsu = DSU::new(n);

    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        if ask[0] == 1 {
            let u = ask[1] - 1;
            let v = ask[2] - 1;
            dsu.unite(u, v);
        } else {
            let u = ask[1] - 1;
            let k = ask[2] - 1;
            let r = dsu.root(u);
            if k < dsu.set[r].len() {
                ans.push(dsu.set[r][k] as i32);
            } else {
                ans.push(-1);
            }
        }
    }

    println!("{}", join(&ans, "\n"));
}

struct DSU {
    par: Vec<usize>,
    siz: Vec<usize>,
    set: Vec<Vec<usize>>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut set = vec![vec![]; n];
        for i in 0..n {
            set[i].push(i + 1);
        }
        Self {
            par: (0..n).collect(),
            siz: vec![1; n],
            set,
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
        if self.siz[u] <= self.siz[v] {
            (u, v) = (v, u);
        }
        self.par[v] = u;
        self.siz[u] += self.siz[v];
        let mut res = vec![];
        res.extend(self.set[u].clone());
        res.extend(self.set[v].clone());
        res.sort();
        res.reverse();
        if res.len() > 10 {
            res.resize(10, 0);
        }
        self.set[u] = res;
        self.set[v].clear();
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
