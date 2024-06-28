#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let tc = read::<usize>();
    for _ in 0..tc {
        let mut s = reads();
        let mut cnt = HashSet::new();
        for i in 0..s.len() {
            cnt.insert(s[i]);
        }

        if cnt.len() == 1 {
            println!("NO");
        } else {
            for i in 0..s.len() {
                if s[i] != s[0] {
                    s.swap(0, i);
                    break;
                }
            }
            println!("YES");
            println!("{}", join(&s, ""));
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
    read::<String>().chars().collect::<_>()
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
