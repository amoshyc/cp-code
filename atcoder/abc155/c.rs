#![allow(unused)]

use std::collections::BTreeMap;

fn main() {
    let n = read::<usize>();
    let mut cnt = BTreeMap::new();
    for _ in 0..n {
        let s = reads();
        *cnt.entry(s).or_insert(0) += 1;
    }

    let mut max = *cnt.values().max().unwrap();
    for (k, v) in cnt {
        if v == max {
            println!("{}", join(&k, ""));
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
