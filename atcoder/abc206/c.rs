#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut ans = 0;
    let mut cnt = HashMap::new();
    for r in 0..n {
        ans += r - cnt.get(&arr[r]).unwrap_or(&0);
        *cnt.entry(arr[r]).or_insert(0) += 1;
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
