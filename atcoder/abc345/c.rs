#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let s = reads();
    let n = s.len();

    let mut cnt = HashMap::new();
    for i in 0..n {
        *cnt.entry(s[i]).or_insert(0) += 1i64;
    }

    let mut ans = 0;
    for i in 'a'..='z' {
        for j in 'a'..='z' {
            if i < j {
                let n_i = cnt.get(&i).unwrap_or(&0);
                let n_j = cnt.get(&j).unwrap_or(&0);
                ans += n_i * n_j;
            }
        }
    }
    if cnt.values().any(|&c| c > 1) {
        ans += 1;
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
