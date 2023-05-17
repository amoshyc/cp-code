#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let n = read::<usize>();
    let s = reads();

    let mut prev = HashMap::new();
    let mut next = HashMap::new();

    next.insert(n + 2, 0);
    prev.insert(0, n + 2);
    
    next.insert(0, n + 3);
    prev.insert(n + 3, 0);

    for i in 1..=n {
        if s[i - 1] == 'L' {
            // p, i - 1, n
            // p, i, i - 1, n
            let p = prev[&(i - 1)];
            next.insert(p, i);
            next.insert(i, i - 1);
            prev.insert(i, p);
            prev.insert(i - 1, i);
        } else {
            // p, i - 1, n
            // p, i - 1, i, n
            let n = next[&(i - 1)];
            next.insert(i - 1, i);
            next.insert(i, n);
            prev.insert(i, i - 1);
            prev.insert(n, i);
        }
    }

    let mut u = n + 2;
    let mut ans = vec![];
    while u != n + 3 {
        ans.push(u);
        u = next[&u];
    }

    println!("{}", join(&ans[1..], " "));
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
