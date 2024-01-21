#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let a = readv::<usize>();
    let mn = *a.iter().min().unwrap();
    let mx = *a.iter().max().unwrap();
    let s: HashSet<_> = a.into_iter().collect();

    for i in mn..=mx {
        if !s.contains(&i) {
            println!("{}", i);
            break;
        }
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
