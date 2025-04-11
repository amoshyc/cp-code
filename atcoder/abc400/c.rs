#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    // To enumerate all 2^a * b^2 without repeat,
    // we can assume that b must be odd number.

    let n = read::<u64>();

    let mut ans = 0;
    for a in (1..).take_while(|a| (1 << a) <= n) {
        // 2^a * b^2 = x
        // => b <= sqrt(n/(2^a)) = rhs
        // we also have b >= 1 and b is odd

        // b[1] = 1
        // b[2] = 3
        // b[3] = 5
        // ...
        // b[k] = previous_odd(rhs) = q
        // => k = (q + 1) // 2

        let rhs = isqrt(n / (1 << a));
        let q = if rhs % 2 == 1 { rhs } else { rhs - 1 };
        let k = (q + 1) / 2;
        ans += k;
    }

    println!("{}", ans);
}

fn isqrt(x: u64) -> u64 {
    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = 1u64 << 32;
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if m.saturating_mul(m) <= x {
            lb = m;
        } else {
            ub = m;
        }
    }
    lb
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
