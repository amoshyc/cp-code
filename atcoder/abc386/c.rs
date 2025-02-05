#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let k = read::<usize>();
    let s = reads();
    let t = reads();

    let mut s = VecDeque::from(s);
    let mut t = VecDeque::from(t);

    while let Some((x, y)) = s.front().zip(t.front()) {
        if x == y {
            s.pop_front();
            t.pop_front();
        } else {
            break;
        }
    }
    while let Some((x, y)) = s.back().zip(t.back()) {
        if x == y {
            s.pop_back();
            t.pop_back();
        } else {
            break;
        }
    }

    if s.len() <= 1 && t.len() <= 1 {
        println!("Yes");
    } else {
        println!("No");
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
