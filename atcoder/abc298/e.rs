#![allow(unused)]

use std::collections::HashMap;

// f(i, j, 0) = Pr(Takahashi wins if Takahashi is at i and Aoki is at j and it's Takahashi turn)
// f(i, j, 1) = Pr(Takahashi wins if Takahashi is at i and Aoki is at j and it's Takahashi turn)

// f(n, _, 0) = 1
// f(_, n, 1) = 0
// f(i, j, 0) = 1 / P * sum( f(min(i + x, n), j, 1) for x in 1..=P )
// f(i, j, 1) = 1 / Q * sum( f(i, min(j + x, n), 0) for x in 1..=Q )
fn f(
    i: u64,
    j: u64,
    t: u64,
    n: u64,
    p: u64,
    q: u64,
    dp: &mut HashMap<(u64, u64, u64), Mint>,
) -> Mint {
    if dp.contains_key(&(i, j, t)) {
        return dp[&(i, j, t)];
    }

    if i == n {
        return Mint(1);
    }
    if j == n {
        return Mint(0);
    }

    let mut val = Mint(0);
    if t == 0 {
        let mut sum = Mint(0);
        for x in 1..=p {
            sum = sum + f(n.min(i + x), j, 1, n, p, q, dp);
        }
        val = sum / Mint(p);
    } else {
        let mut sum = Mint(0);
        for x in 1..=q {
            sum = sum + f(i, n.min(j + x), 0, n, p, q, dp);
        }
        val = sum / Mint(q);
    }

    dp.insert((i, j, t), val);
    val
}

fn main() {
    let inp = readv::<u64>();
    let (n, a, b, p, q) = (inp[0], inp[1], inp[2], inp[3], inp[4]);
    let mut dp: HashMap<(u64, u64, u64), Mint> = HashMap::new();
    println!("{}", f(a, b, 0, n, p, q, &mut dp));
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
