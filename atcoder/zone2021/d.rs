#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let s = reads();
    let mut t = VecDeque::<char>::new();
    let mut f = true;
    for &c in &s {
        if c == 'R' {
            f = !f;
        } else {
            if f {
                t.push_back(c);
            } else {
                t.push_front(c);
            }
        }
    }

    let mut t: Vec<char> = t.iter().copied().collect();
    if !f {
        t.reverse();
    }

    let mut ans = vec![];
    for &c in &t {
        if ans.len() == 0 {
            ans.push(c);
            continue;
        }
        if c == *ans.last().unwrap() {
            ans.pop();
            continue;
        } else {
            ans.push(c);
        }
    }
    println!("{}", join(&ans, ""));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
