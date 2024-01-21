#![allow(unused)]

use std::cmp::Reverse;

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for i in 0..n {
        let s = reads();
        let w = (0..n).filter(|&i| s[i] == 'o').count();
        arr.push((w, i));
    }
    arr.sort_by_key(|&(w, i)| (Reverse(w), i));
    let ans = mapv(&arr, |&(w, i)| i + 1);
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
