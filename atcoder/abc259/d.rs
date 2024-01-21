#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let inp = readv::<i64>();
    let (sx, sy, tx, ty) = (inp[0], inp[1], inp[2], inp[3]);
    let mut circles = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        circles.push((inp[0], inp[1], inp[2]));
    }

    let mut dsu = DSU::new(n);
    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1, r1) = circles[i];
            let (x2, y2, r2) = circles[j];
            let d2 = (x1 - x2).pow(2) + (y1 - y2).pow(2);
            if d2 < (r1 - r2).pow(2) {
                continue;
            }
            if d2 > (r1 + r2).pow(2) {
                continue;
            }
            dsu.unite(i, j);
        }
    }

    let s_index = circles
        .iter()
        .position(|&(x, y, r)| (sx - x).pow(2) + (sy - y).pow(2) == r.pow(2))
        .unwrap();
    let t_index = circles
        .iter()
        .position(|&(x, y, r)| (tx - x).pow(2) + (ty - y).pow(2) == r.pow(2))
        .unwrap();
    
    if dsu.same(s_index, t_index) {
        println!("Yes");
    } else {
        println!("No");
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

    fn unite(&mut self, mut a: usize, mut b: usize) {
        a = self.root(a);
        b = self.root(b);
        if a == b {
            return;
        }
        if self.siz[a] < self.siz[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.par[a] = b;
        self.siz[b] += self.siz[a];
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
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
