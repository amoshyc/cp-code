#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w, y) = (inp[0], inp[1], inp[2]);
    let mut arr = vec![];
    for _ in 0..h {
        arr.push(readv::<usize>());
    }

    let mut sea = h * w;
    let mut dsu = DSU::new(h * w + 1);
    let mut inv = vec![vec![]; 100_000 + 1];
    for r in 0..h {
        for c in 0..w {
            inv[arr[r][c]].push((r, c));
        }
    }

    let mut ans = vec![];
    for curr_y in 1..=y {
        for &(r, c) in inv[curr_y].iter() {
            if r == 0 || r == h - 1 || c == 0 || c == w - 1 {
                dsu.unite(r * w + c, sea);
            }
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = r.checked_add_signed(dr).unwrap_or(h);
                let nc = c.checked_add_signed(dc).unwrap_or(w);
                if nr < h && nc < w && arr[nr][nc] <= arr[r][c] {
                    dsu.unite(r * w + c, nr * w + nc);
                }
            }
        }
        ans.push(h * w + 1 - dsu.size(sea));
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
