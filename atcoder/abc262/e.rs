#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);

    let mut deg = vec![0; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        deg[u] += 1;
        deg[v] += 1;
    }

    let mut cnt = vec![0; 2];
    for d in deg {
        cnt[d % 2] += 1;
    }
    
    let tool = CombTool::new(n);
    let mut ans = Mint(0);
    for i in (0..=k).step_by(2) {
        // Choose i vertices with odd degree and K - i vertices with even degree.
        if (0..=cnt[1]).contains(&i) && (0..=cnt[0]).contains(&(k - i)) {
            ans = ans + tool.comb(cnt[1], i) * tool.comb(cnt[0], k - i);
        }
    }
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
impl std::fmt::Display for Mint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct CombTool {
    fact: Vec<Mint>,
    finv: Vec<Mint>,
}

impl CombTool {
    fn new(n: usize) -> CombTool {
        let mut fact = vec![Mint(1); n + 1];
        let mut finv = vec![Mint(1); n + 1];
        for i in 1..=n {
            fact[i] = fact[i - 1] * Mint(i as u64);
        }
        finv[n] = fact[n].inv();
        for i in (1..n).rev() {
            finv[i] = finv[i + 1] * Mint((i + 1) as u64);
        }
        CombTool { fact, finv }
    }
    fn comb(&self, a: usize, b: usize) -> Mint {
        self.fact[a] * self.finv[b] * self.finv[a - b]
    }
    fn perm(&self, a: usize, b: usize) -> Mint {
        self.fact[a] * self.finv[a - b]
    }
    fn hcomb(self, a: usize, b: usize) -> Mint {
        self.comb(a + b - 1, b)
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
