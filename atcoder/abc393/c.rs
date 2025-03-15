#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut set = HashSet::new();
    let mut ans = 0;
    for _ in 0..m {
        let uv = readv::<usize>();
        let (u, v) = (uv[0], uv[1]);
        let e = (u.min(v), u.max(v));
        if set.contains(&e) || u == v {
            ans += 1;
        } else {
            set.insert(e);
        }
    }
    println!("{}", ans);
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
