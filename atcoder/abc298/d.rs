#![allow(unused)]

fn main() {
    let q = read::<usize>();

    let m = 998244353;
    let mut arr = vec![1];
    let mut s = 0;
    let mut val = 1 as u64;

    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<u64>();

        if inp[0] == 1 {
            arr.push(inp[1]);
            val = (val * 10 % m + inp[1]) % m;
        } else if inp[0] == 2 {
            let e = arr.len() - s - 1;
            val = (val + m - arr[s] * powmod(10, e as u64, m) % m) % m;
            s += 1;
        } else {
            ans.push(val);
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn powmod(a: u64, mut b: u64, m: u64) -> u64 {
    let mut base = a % m;
    let mut res = 1;
    while b != 0 {
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
