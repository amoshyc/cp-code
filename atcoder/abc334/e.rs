#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![];
    for r in 0..h {
        arr.push(reads());
    }

    let mut dsu = DSU::new(h * w);
    let mut cnt = 0;
    for r in 0..h {
        for c in 0..w {
            for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nr = r.checked_add_signed(dr).unwrap_or(h);
                let nc = c.checked_add_signed(dc).unwrap_or(w);
                if nr < h && nc < w && arr[r][c] == arr[nr][nc] {
                    dsu.unite(r * w + c, nr * w + nc);
                }
            }
            if arr[r][c] == '.' {
                cnt += 1;
            }
        }
    }

    let mut set = std::collections::HashSet::new();
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == '#' {
                set.insert(dsu.root(r * w + c));
            }
        }
    }

    let mut g = Mint(set.len() as u64);
    let mut ans = Mint(0);
    for r in 0..h {
        for c in 0..w {
            if arr[r][c] == '.' {
                let mut set = std::collections::HashSet::new();
                for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nr = r.checked_add_signed(dr).unwrap_or(h);
                    let nc = c.checked_add_signed(dc).unwrap_or(w);
                    if nr < h && nc < w && arr[nr][nc] == '#' {
                        set.insert(dsu.root(nr * w + nc));
                    }
                }
                ans += (g - Mint(set.len() as u64) + Mint(1));
            }
        }
    }
    ans /= Mint(cnt);
    println!("{}", ans);
}

#[derive(Debug, Copy, Clone)]
struct Mint(u64);

impl Mint {
    const MOD: u64 = 998_244_353;
    fn inv(&self) -> Mint {
        let mut a = self.0 as i64;
        let mut b = Mint::MOD as i64;
        let mut p = 1 as i64;
        let mut q = 0 as i64;
        while b != 0 {
            let (c, r) = (a / b, a % b);
            a = b;
            b = r;
            let tmp = p - c * q;
            p = q;
            q = tmp;
        }
        Mint(p.rem_euclid(Mint::MOD as i64) as u64)
    }
    fn pow(&self, mut b: u64) -> Mint {
        let mut base = *self;
        let mut res = Mint(1);
        while b != 0 {
            if b & 1 == 1 {
                res = res * base;
            }
            base = base * base;
            b >>= 1;
        }
        res
    }
}
impl Default for Mint {
    fn default() -> Self {
        Self(0)
    }
}
impl std::ops::Add for Mint {
    type Output = Mint;
    fn add(self, rhs: Mint) -> Mint {
        Mint((self.0 + rhs.0) % Mint::MOD)
    }
}
impl std::ops::Sub for Mint {
    type Output = Mint;
    fn sub(self, rhs: Mint) -> Mint {
        Mint((self.0 + Mint::MOD - rhs.0) % Mint::MOD)
    }
}
impl std::ops::Mul for Mint {
    type Output = Mint;
    fn mul(self, rhs: Mint) -> Mint {
        Mint(self.0 * rhs.0 % Mint::MOD)
    }
}
impl std::ops::Div for Mint {
    type Output = Mint;
    fn div(self, rhs: Mint) -> Mint {
        self * rhs.inv()
    }
}
impl std::ops::AddAssign for Mint {
    fn add_assign(&mut self, rhs: Self) {
        *self = Mint((self.0 + rhs.0) % Mint::MOD);
    }
}
impl std::ops::SubAssign for Mint {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Mint((self.0 + Mint::MOD - rhs.0) % Mint::MOD);
    }
}
impl std::ops::MulAssign for Mint {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Mint((self.0 * rhs.0) % Mint::MOD);
    }
}
impl std::ops::DivAssign for Mint {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self * rhs.inv();
    }
}
impl std::fmt::Display for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
