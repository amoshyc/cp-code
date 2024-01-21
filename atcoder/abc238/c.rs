#![allow(unused)]

const MOD: u64 = 998244353;

fn main() {
    let n = read::<u64>();
    let k = n.to_string().len() as u32;

    // i digit number
    // f(10...0) = 1
    // f(99...9) = 10**i - 10**(i - 1)

    let mut ans = 0;
    for i in 1..=k {
        let lb = 1;
        let ub = (10u64.pow(i).min(n + 1) - 10u64.pow(i - 1)) % MOD;
        let val = (lb + ub) % MOD * ub % MOD * powmod(2, MOD - 2, MOD) % MOD;
        ans = (ans + val) % MOD;
    }

    println!("{}", ans);
}

fn powmod(a: u64, mut b: u64, m: u64) -> u64 {
    let mut base = a;
    let mut res = 1 as u64;
    while b > 0 {
        if b & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        b >>= 1;
    }
    res
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
