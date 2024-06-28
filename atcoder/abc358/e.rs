#![allow(unused)]

fn main() {
    // dp[i, j] = number of string of length j that uses 'a'..=('a' + i)
    // dp[i, j] = dp[i - 1, j] + comb(j, 1) * dp[i - 1][j - 1] + comb(j, 2) * dp[i - 1][j - 2] + ...

    let k = read::<usize>();
    let cnt = readv::<usize>();
    let tool = CombTool::new(k);

    let mut dp = vec![vec![Mint(0); k + 1]; 26];

    for j in 0..=cnt[0].min(k) {
        dp[0][j] = Mint(1);
    }

    for i in 1..26 {
        for j in 0..=k {
            for l in 0..=cnt[i].min(j) {
                dp[i][j] = dp[i][j] + tool.comb(j, l) * dp[i - 1][j - l];
            }
        }
    }

    let mut ans = Mint(0);
    for j in 1..=k {
        ans = ans + dp[25][j];
    }
    println!("{}", ans);
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
