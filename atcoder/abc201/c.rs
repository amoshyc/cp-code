#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let s = reads();

    let mut set = HashSet::new();
    for i in 0..10 {
        if s[i] == 'o' {
            set.insert(i);
        }
    }

    let mut ans = 0;
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    let mut ok = true;
                    ok &= s[a] != 'x' && s[b] != 'x' && s[c] != 'x' && s[d] != 'x';
                    ok &= set.iter().all(|x| *x == a || *x == b || *x == c || *x == d);
                    if ok {
                        ans += 1;
                    }
                }
            }
        }
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
