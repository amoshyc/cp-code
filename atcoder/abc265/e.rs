#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let inp = readv::<i64>();
    let (a, b, c, d, e, f) = (inp[0], inp[1], inp[2], inp[3], inp[4], inp[5]);
    let mut blocks = std::collections::HashSet::new();
    for _ in 0..m {
        let inp = readv::<i64>();
        let (x, y) = (inp[0], inp[1]);
        blocks.insert((x, y));
    }

    // dp[i][j][k] = number of ways to go to (x, y) where
    //      x = a * i + c * j + e * k
    //      y = b * i + d * j + f * k
    // dp[0][0][0] = 1
    // dp[i][j][k] = dp[i - 1][j][k] + dp[i][j - 1][k] + dp[i][j][k - 1]

    let mut dp = vec![vec![vec![Mint(0); n + 1]; n + 1]; n + 1];
    dp[0][0][0] = Mint(1);
    for i in 0..=n {
        for j in 0..=(n - i) {
            for k in 0..=(n - i - j) {
                let x = a * (i as i64) + c * (j as i64) + e * (k as i64);
                let y = b * (i as i64) + d * (j as i64) + f * (k as i64);
                if (i + j + k) == 0 || blocks.contains(&(x, y)) {
                    continue;
                }
                dp[i][j][k] = Mint(0);
                if i >= 1 {
                    dp[i][j][k] = dp[i][j][k] + dp[i - 1][j][k];
                }
                if j >= 1 {
                    dp[i][j][k] = dp[i][j][k] + dp[i][j - 1][k];
                }
                if k >= 1 {
                    dp[i][j][k] = dp[i][j][k] + dp[i][j][k - 1];
                }
            }
        }
    }

    let mut ans = Mint(0);
    for i in 0..=n {
        for j in 0..=(n - i) {
            ans = ans + dp[i][j][n - i - j];
        }
    }

    println!("{}", ans);
}

#[derive(Debug, Copy, Clone)]
struct Mint(u64);

impl Mint {
    const MOD: u64 = 998244353;
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
