#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut last = HashMap::new();
    let mut ans = vec![-1; n];
    for i in 0..n {
        ans[i] = last.get(&arr[i]).unwrap_or(&-2) + 1;
        last.insert(arr[i], i as i64);
    }
    println!("{}", join(&ans, " "));
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
