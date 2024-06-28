#![allow(unused)]

fn main() {
    // k * D(x) = D(k * x)
    // => f(i) = number of 0-digit, 1-digit, ..., i-digit number zzz...z
    //           satisfing 0 <= z <= floor_div(z, k)
    //         = (floor_div(z, k) + 1)^i
    // answer = f(r) - f(l)
    let tc = read::<usize>();
    for _ in 0..tc {
        let inp = readv::<u64>();
        let (l, r, k) = (inp[0], inp[1], inp[2]);
        let m = 10u64.pow(9) + 7;

        let f = |i: u64| powmod(9 / k + 1, i, m);
        let ans = (f(r) + m - f(l)) % m;
        println!("{}", ans);
    }
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
