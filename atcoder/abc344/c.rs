#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let _ = read::<usize>();
    let a = readv::<i64>();
    let _ = read::<usize>();
    let b = readv::<i64>();
    let _ = read::<usize>();
    let c = readv::<i64>();

    let mut set = HashSet::new();
    for a in a.iter() {
        for b in b.iter() {
            for c in c.iter() {
                set.insert(a + b + c);
            }
        }
    }

    let q = read::<usize>();
    let x = readv::<i64>();
    let ans = mapv(&x, |&x| if set.contains(&x) { "Yes" } else { "No" });
    println!("{}", join(&ans, "\n"));
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
    read::<String>().chars().collect::<_>()
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
