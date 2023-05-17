#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut ans = Mint(0);
    for d in 1..=n {
        // dp[i][j][k] = number of ways to choose j items from A[..=i] such that the sum % d = k
        // dp[0][0][0] = 1
        // dp[0][1][A[0] % d] = 1
        // dp[i][j][k] = dp[i - 1][j][k] + dp[i - 1][j - 1][(k - A[i]) % d]
        let mut dp = vec![vec![vec![Mint(0); d]; d + 1]; n];
        dp[0][0][0] = Mint(1);
        dp[0][1][arr[0] % d] = Mint(1);
        for i in 1..n {
            for j in 0..=d {
                for k in 0..d {
                    let item1 = dp[i - 1][j][k];
                    let item2 = if j >= 1 {
                        let prev_k = (k as i32 - arr[i] as i32).rem_euclid(d as i32) as usize;
                        dp[i - 1][j - 1][prev_k]
                    } else {
                        Mint(0)
                    };
                    dp[i][j][k] = item1 + item2;
                }
            }
        }
        ans = ans + dp[n - 1][d][0];
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
