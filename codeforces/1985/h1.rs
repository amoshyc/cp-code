#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let inp = readv::<usize>();
        let (h, w) = (inp[0], inp[1]);
        let mut arr = vec![];
        for _ in 0..h {
            arr.push(reads());
        }

        let mut dsu = DSU::new(h * w);
        for r in 0..h {
            for c in 0..w {
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nr = r.checked_add_signed(dr).unwrap_or(!0);
                    let nc = c.checked_add_signed(dc).unwrap_or(!0);
                    if nr < h && nc < w && arr[r][c] == '#' && arr[nr][nc] == '#' {
                        dsu.unite(r * w + c, nr * w + nc);
                    }
                }
            }
        }

        let mut ans = 0;
        for r in 0..h {
            for c in 0..w {
                if arr[r][c] == '#' {
                    ans = ans.max(dsu.size(r * w + c));
                }
            }
        }

        for c in 0..w {
            let mut gids = HashSet::new();
            let mut cnt = 0;
            for dc in [-1, 0, 1] {
                let cc = c.checked_add_signed(dc).unwrap_or(w);
                if cc < w {
                    for r in 0..h {
                        if arr[r][cc] == '#' {
                            let gid = dsu.root(r * w + cc);
                            let siz = dsu.size(r * w + cc);
                            if !gids.contains(&gid) {
                                gids.insert(gid);
                                cnt += siz;
                            }
                        }
                    }
                }
            }
            for r in 0..h {
                if arr[r][c] == '.' {
                    cnt += 1;
                }
            }
            ans = ans.max(cnt);
        }

        for r in 0..h {
            let mut gids = HashSet::new();
            let mut cnt = 0;
            for dr in [-1, 0, 1] {
                let rr = r.checked_add_signed(dr).unwrap_or(h);
                if rr < h {
                    for c in 0..w {
                        if arr[rr][c] == '#' {
                            let gid = dsu.root(rr * w + c);
                            let siz = dsu.size(rr * w + c);
                            if !gids.contains(&gid) {
                                gids.insert(gid);
                                cnt += siz;
                            }
                        }
                    }
                }
            }
            for c in 0..w {
                if arr[r][c] == '.' {
                    cnt += 1;
                }
            }
            ans = ans.max(cnt);
        }
        println!("{}", ans);
    }
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
