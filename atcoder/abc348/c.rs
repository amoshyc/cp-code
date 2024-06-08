#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let mut min: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let inp = readv::<usize>();
        let (a, c) = (inp[0], inp[1]);
        if min.contains_key(&c) {
            min.insert(c, min[&c].min(a));
        } else {
            min.insert(c, a);
        }
    }

    let ans = min.values().max().unwrap();
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
