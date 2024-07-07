#![allow(unused)]

use std::collections::{HashMap, VecDeque};

fn main() {
    let n = read::<usize>();
    let mut s = reads();
    let mut t = reads();

    s.push('.');
    s.push('.');
    t.push('.');
    t.push('.');

    let mut dis = HashMap::new();
    let mut que = VecDeque::new();

    dis.insert(s.clone(), 0);
    que.push_back(s);

    while let Some(u) = que.pop_front() {
        if u == t {
            println!("{}", dis[&u]);
            return;
        }

        let p = (0..u.len()).find(|&i| u[i] == '.').unwrap();
        for i in (0..u.len() - 1) {
            if u[i] != '.' && u[i + 1] != '.' {
                let mut v = u.clone();
                v.swap(i, p);
                v.swap(i + 1, p + 1);
                if !dis.contains_key(&v) {
                    dis.insert(v.clone(), dis[&u] + 1);
                    que.push_back(v);
                }
            }
        }
    }

    println!("-1");
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
