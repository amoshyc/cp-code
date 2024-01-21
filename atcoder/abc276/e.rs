#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let mut dsu = DSU::new(n * m);
    for r in 0..n {
        for c in 1..m {
            if (arr[r][c - 1], arr[r][c]) == ('.', '.') {
                let u = r * m + c - 1;
                let v = r * m + c;
                dsu.unite(u, v);
            }
        }
    }
    for c in 0..m {
        for r in 1..n {
            if (arr[r - 1][c], arr[r][c]) == ('.', '.') {
                let u = (r - 1) * m + c;
                let v = r * m + c;
                dsu.unite(u, v);
            }
        }
    }

    let mut adjs = vec![];
    for r in 0..n {
        for c in 0..m {
            if arr[r][c] == 'S' {
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nr = r.checked_add_signed(dr).unwrap_or(n);
                    let nc = c.checked_add_signed(dc).unwrap_or(m);
                    if nr < n && nc < m && arr[nr][nc] == '.' {
                        adjs.push((nr, nc));
                    }
                }
            }
        }
    }

    let mut ans = "No";
    for i in 0..adjs.len() {
        for j in (i + 1)..adjs.len() {
            let (ri, ci) = adjs[i];
            let (rj, cj) = adjs[j];
            let u = ri * m + ci;
            let v = rj * m + cj;
            if dsu.same(u, v) {
                ans = "Yes";
            }
        }
    }

    println!("{}", ans);
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
