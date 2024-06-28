#![allow(unused)]

use std::collections::HashMap;

fn solve() -> String {
    let inp = readv::<usize>();
    let (n, dr, dc) = (inp[0], inp[1] as i64, inp[2] as i64);
    let arr = readv::<i64>();

    let mut cnt = HashMap::new();
    for &x in arr.iter() {
        *cnt.entry(x).or_insert(0) += 1;
    }

    let a11 = *arr.iter().min().unwrap();
    *cnt.entry(a11).or_insert(0) -= 1;

    for r in 0..n {
        for c in 0..n {
            if (r, c) == (0, 0) {
                continue;
            }
            let x = a11 + r as i64 * dr + c as i64 * dc;
            let e = cnt.entry(x).or_insert(0);
            if *e == 0 {
                return "No".to_string();
            } else {
                *e -= 1;
            }
        }
    }

    "Yes".to_string()
}

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        ans.push(solve());
    }
    println!("{}", join(&ans, "\n"));
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
