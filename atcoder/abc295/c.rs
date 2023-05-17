#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let arr = readv::<u32>();
    let mut cnt = HashMap::new();
    for &x in arr.iter() {
        *cnt.entry(x).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (k, v) in cnt.iter() {
        ans += v / 2;
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
