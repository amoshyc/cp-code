#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let inp = readv::<String>();
    let mut s = HashSet::new();
    s.insert("and");
    s.insert("not");
    s.insert("that");
    s.insert("the");
    s.insert("you");
    if inp.iter().any(|w| s.contains(&w[..])) {
        println!("Yes");
    } else {
        println!("No");
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}