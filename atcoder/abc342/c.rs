#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let s = reads();

    let mut f = HashMap::new();
    for c in 'a'..='z' {
        f.insert(c, c);
    }

    let q = read::<usize>();
    for _ in 0..q {
        let inp = reads();
        let (a, b) = (inp[0], inp[2]);
        for (k, v) in f.iter_mut() {
            if *v == a {
                *v = b;
            }
        }
    }

    let ans = mapv(&s, |c| f[c]);
    println!("{}", join(&ans, ""));
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
