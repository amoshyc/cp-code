#![allow(unused)]

use std::collections::HashMap;

// f(i, j, 0) = Pr(Takahashi wins if Takahashi is at i and Aoki is at j and it's Takahashi turn)
// f(i, j, 1) = Pr(Takahashi wins if Takahashi is at i and Aoki is at j and it's Aoki turn)

// f(n, _, 0) = 1
// f(_, n, 1) = 0
// f(i, j, 0) = 1 / P * sum( f(min(i + x, n), j, 1) for x in 1..=P )
// f(i, j, 1) = 1 / Q * sum( f(i, min(j + x, n), 0) for x in 1..=Q )

fn main() {
    let inp = readv::<usize>();
    let (n, a, b, p, q) = (inp[0], inp[1], inp[2], inp[3], inp[4]);
    let mut dp = vec![vec![vec![Mint(0); 2]; n + 1]; n + 1];

    for j in 0..=n {
        dp[n][j][0] = Mint(1);
        dp[n][j][1] = Mint(1);
    }
    for i in 0..=n {
        dp[i][n][0] = Mint(0);
        dp[i][n][1] = Mint(0);
    }

    for i in (a..n).rev() {
        for j in (b..n).rev() {
            // t = 0
            dp[i][j][0] = Mint(0);
            for x in 1..=p {
                dp[i][j][0] = dp[i][j][0] + dp[n.min(i + x)][j][1];
            }
            dp[i][j][0] = dp[i][j][0] / Mint(p as u64);
            // t = 1
            dp[i][j][1] = Mint(0);
            for x in 1..=q {
                dp[i][j][1] = dp[i][j][1] + dp[i][n.min(j + x)][0];
            }
            dp[i][j][1] = dp[i][j][1]  / Mint(q as u64);
        }
    }

    println!("{}", dp[a][b][0]);
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
