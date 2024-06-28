#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let s = reads();

    // dp[i, m] = number of valid T[..=i] such that T[i - k + 1..=i] = m
    // dp[i, m] = 0 if m is palindrome
    // answer = sum(dp[n - 1, m] for m in 0..(1 << k))

    // (k = 3)           i
    //         x x x x x x x
    // curr:         ^^^^^
    // next:           ^^^^^
    // dp[i, m] -> dp[i + 1, m']

    let mut is_palindrome = vec![true; (1 << k)];
    for m in 0..(1 << k) {
        for i in 0..(k / 2) {
            if nth_bit(m, i) != nth_bit(m, k - 1 - i) {
                is_palindrome[m] = false;
                break;
            }
        }
    }

    let mut dp = vec![vec![Mint(0); (1 << k)]; n];

    // base case: dp[k - 1, m]
    for m in 0..(1 << k) {
        if is_palindrome[m] {
            continue;
        }
        // if m doesn't match s[..=(k - 1)], then dp[k - 1, m] = 0
        // otherwise m is a valid T[..=(k - 1)]
        if (0..k).any(|i| {
            let case1 = (s[i], nth_bit(m, i)) == ('A', 1);
            let case2 = (s[i], nth_bit(m, i)) == ('B', 0);
            case1 || case2
        }) {
            continue;
        }
        dp[k - 1][m] = Mint(1);
    }

    for i in (k - 1)..(n - 1) {
        for m in 0..(1 << k) {
            for c in ['A', 'B'] {
                if s[i + 1] == c || s[i + 1] == '?' {
                    let mut new_m = m >> 1;
                    if c == 'B' {
                        new_m |= (1 << (k - 1));
                    }
                    if !is_palindrome[new_m] {
                        dp[i + 1][new_m] = dp[i + 1][new_m] + dp[i][m];
                    }
                }
            }
        }
    }

    let mut ans = Mint(0);
    for m in 0..(1 << k) {
        ans = ans + dp[n - 1][m];
    }
    println!("{}", ans);
}

fn nth_bit(m: usize, n: usize) -> usize {
    (m >> n) & 1
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
