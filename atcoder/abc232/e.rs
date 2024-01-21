#![allow(unused)]

const MOD: u64 = 998244353;

fn main() {
    let inp = readv::<u64>();
    let (h, w, k) = (inp[0], inp[1], inp[2] as usize);
    let inp = readv::<usize>();
    let (sr, sc) = (inp[0], inp[1]);
    let (gr, gc) = (inp[2], inp[3]);

    // dp[i, 0/1] = number of ways to move i times from s and stops at other / g
    // dp[0][[s == g]] = 1
    // Consider the place after moving i times.
    // if stops at other, then previous place can be other or g.
    //   if previous place is other, then each of other place has n - 2 ways to move.
    //   if previous place is g, then it has n - 1 ways to move.
    // if stops at g, then previous place is other, each place has 1 way to move.

    let mut dp = vec![vec![Mint(0); 2]; k + 1];
    if sc == gc {
        dp[0][1] = Mint(1);
    } else {
        dp[0][0] = Mint(1);
    }
    for i in 1..=k {
        dp[i][0] = dp[i - 1][0] * Mint(w - 2) + dp[i - 1][1] * Mint(w - 1);
        dp[i][1] = dp[i - 1][0];
    }

    let mut ep = vec![vec![Mint(0); 2]; k + 1];
    if sr == gr {
        ep[0][1] = Mint(1);
    } else {
        ep[0][0] = Mint(1);
    }
    for i in 1..=k {
        ep[i][0] = ep[i - 1][0] * Mint(h - 2) + ep[i - 1][1] * Mint(h - 1);
        ep[i][1] = ep[i - 1][0];
    }

    let tool = CombTool::new(k);
    let mut ans = Mint(0);
    for hori in 0..=k {
        let vert = k - hori;
        ans += dp[hori][1] * ep[vert][1] * tool.comb(k, hori);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
