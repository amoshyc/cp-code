#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let a = readv::<usize>();
    let m = read::<usize>();
    let b = readv::<usize>();
    let x = read::<usize>();

    let mut traps = HashSet::new();
    for t in b {
        traps.insert(t);
    }

    let mut stack = vec![0];
    let mut vis = vec![false; 100_001];
    vis[0] = true;
    while let Some(u) = stack.pop() {
        if traps.contains(&u) {
            continue;
        }
        if u == x {
            break;
        }
        for &d in a.iter() {
            let v = u + d;
            if v < vis.len() && !vis[v] {
                vis[v] = true;
                stack.push(v);
            }
        }
    }

    println!("{}", if vis[x] { "Yes" } else { "No" });
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
