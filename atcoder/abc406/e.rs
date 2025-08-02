#![allow(unused)]

fn main() {
    let t = read::<usize>();
    let mut ans = vec![Mint(0); t];
    for i in 0..t {
        let inp = readv::<i64>();
        let (mut n, k) = (inp[0], inp[1] as usize);

        let mut digits = vec![];
        for _ in 0..60 {
            digits.push((n & 1) as usize);
            n >>= 1;
        }

        ans[i] = solve(digits, k);
    }

    println!("{}", join(&ans, "\n"));
}

fn solve(n: Vec<usize>, p: usize) -> Mint {
    // (binary)
    // n[4] n[3] n[2] n[1] n[0]
    //  1    0    1    0    0    (20)
    // x[4] x[3] x[2] x[1] x[0]

    // dp[i, s, f] = number of valid ways to fill x[i - 1..=0] that
    // 1. digit sum = s
    // 2. (f = 0) x[i - 1..=0] <= n[i - 1..=0]
    //    (f = 1) x[i - 1..=0] > n[i - 1..=0]

    // dp[i, s, f] -> dp[i + 1, (s + b), f'] where b = 0 or 1

    let m = 60;
    let mut dp = vec![vec![vec![Mint(0), Mint(0)]; p + 1]; m + 1];
    let mut ans = vec![vec![vec![Mint(0), Mint(0)]; p + 1]; m + 1];
    dp[0][0][0] = Mint(1);
    for i in 0..m {
        for s in 0..=p {
            for f in 0..2 {
                for b in 0..2 {
                    let new_s = s + b;
                    if new_s > p {
                        continue;
                    }
                    let new_f = ((b > n[i]) || (b == n[i] && f == 1)) as usize;
                    dp[i + 1][new_s][new_f] = dp[i + 1][new_s][new_f] + dp[i][s][f];

                    let delta = dp[i][s][f] * Mint(2).pow(i as u64) * Mint(b as u64);
                    ans[i + 1][new_s][new_f] = ans[i + 1][new_s][new_f] + ans[i][s][f] + delta;
                }
            }
        }
    }

    ans[m][p][0]
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
    std::io::stdin().read_line(&mut s);
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
