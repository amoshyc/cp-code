#![allow(unused)]

const MOD: u64 = 998_244_353;

fn main() {
    let sieve = SieveOfEratosthenes::new(1_000_000);

    let inp = readv::<u64>();
    let (a, b) = (inp[0], inp[1]);

    if b == 0 {
        println!("{}", 0);
        return;
    }

    // The product of the factor of a^b is a^(b * d(a^b) / 2)
    // The problem ask for the maximum time it can be divided by a, that is
    // if b * d(a^b) is odd, (d(a^b) - 1) / 2
    // if b * d(a^b) is even, d(a^b) / 2

    let mut d = Mint(1); // d(a ^ b)
    let mut is_odd = b % 2 == 1;
    for (p, e) in sieve.factorize(a) {
        d *= Mint(b % MOD) * Mint(e % MOD) + Mint(1);
        is_odd &= (e + 1) % 2 == 1;
    }

    if is_odd {
        println!("{}", (Mint(b % MOD) * d - Mint(1)) / Mint(2));
    } else {
        println!("{}", Mint(b % MOD) * d / Mint(2))
    }
}

struct SieveOfEratosthenes {
    primes: Vec<u64>,
}

impl SieveOfEratosthenes {
    fn new(max_val: usize) -> Self {
        let mut is_prime = vec![true; max_val + 1];
        let mut primes = vec![];
        for i in 2..=max_val {
            if is_prime[i] {
                primes.push(i as u64);
                let mut j = i * i;
                while j <= max_val {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
        Self { primes }
    }

    fn factorize(&self, mut x: u64) -> Vec<(u64, u64)> {
        assert!(x > 1);
        let mut res = vec![];
        for &p in self.primes.iter() {
            let mut exp = 0;
            while x % p == 0 {
                exp += 1;
                x = x / p;
            }
            if exp > 0 {
                res.push((p, exp))
            }
            if p * p > x {
                break;
            }
        }
        if x > 1 {
            res.push((x, 1));
        }
        res
    }
}

// x = p^a q^b r^c

// Number of factors of x:
//    d(x) = (a + 1)(b + 1)(c + 1)

// Sum of factors of x = ABC where
//    A = (p^(a + 1) - 1) / (p - 1)
//    B = (q^(b + 1) - 1) / (q - 1)
//    C = (r^(c + 1) - 1) / (r - 1)

// Product of factors of x:
//    x^(d(x) / 2)
// Note that d(x) / 2 may not be integer
// Take the factors of 36 as example:
// 1  2  3  4  6
// 36 18 12 9
// There are d(x) / 2 pairs which have product 36

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
    read::<String>().chars().collect::<Vec<char>>()
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
