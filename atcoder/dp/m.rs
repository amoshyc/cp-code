#![allow(unused)]

fn main() {
    // dp[i, j] = number of ways to share j candies among people 0..=i
    // dp[i, j] = sum(dp[i - 1, j - k] for k in 0..=arr[i])
    // dp[i, j] = sum(dp[i - 1, x] for x in (j - arr[i])..=j)

    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let mut dp = vec![vec![Mint(0); k + 1]; n];
    for j in 0..=arr[0] {
        dp[0][j] = Mint(1);
    }
    for i in 1..n {
        let pref = build(&dp[i - 1]);
        for j in 0..=k {
            let lb = if j >= arr[i] { j - arr[i] } else { 0 };
            let ub = j;
            dp[i][j] = query(&pref, lb, ub + 1);
        }
    }
    println!("{}", dp[n - 1][k]);
}

fn build<T: Copy + std::ops::Add<Output = T>>(arr: &[T]) -> Vec<T> {
    let mut pref = vec![];
    pref.push(arr[0]);
    for i in 1..arr.len() {
        pref.push(pref[i - 1] + arr[i]);
    }
    pref
}

// i..j
fn query<T: Default + Copy + std::ops::Sub<Output = T>>(pref: &[T], i: usize, j: usize) -> T {
    if i == j {
        return T::default();
    }
    let mut res = pref[j - 1];
    if i > 0 {
        res = res - pref[i - 1];
    }
    res
}

#[derive(Debug, Copy, Clone)]
struct Mint(u64);

impl Mint {
    const MOD: u64 = 1_000_000_007;
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
        Mint(0)
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
