#![allow(unused)]

// [Problem]
// Teppei has created a programming contest site called "LowCoder" and launched the service. As of today, there are no users on LowCoder.
// Starting from today,
// On the ith day (1 ≤ i ≤ N), a user who desires the username Si will submit a registration request.
// If a user with the username Si already exists at the time of application, the registration request will be ignored.
// If no such user exists, the registration request will be accepted, and that user will be registered on LowCoder.
// Please find out on which day the registration request will be accepted.

// [Solution]
// Use a dict/map.

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let mut ans = vec![];
    let mut map = HashMap::new();
    for i in 1..=n {
        let s = read::<String>();
        if map.contains_key(&s) {
            continue;
        } else {
            map.insert(s, i);
            ans.push(i);
        }
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
