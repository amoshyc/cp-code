#![allow(unused)]

use std::collections::BTreeMap;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut freq = BTreeMap::new();
    let arr = readv::<i64>();
    for i in 0..n {
        *freq.entry(arr[i]).or_insert(0) += 1;
    }

    for _ in 0..m {
        let inp = readv::<i64>();
        let (b, c) = (inp[0], inp[1]);
        *freq.entry(c).or_insert(0) += b;
    }

    let mut ans = 0;
    let mut cnt = 0;
    for (&v, &c) in freq.iter().rev() {
        let k = (n as i64 - cnt).min(c);
        ans += v * k;
        cnt += k;
    }

    println!("{ans}");
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
