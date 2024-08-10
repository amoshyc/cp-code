#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let q = read::<usize>();
    let mut cnt = HashMap::new();

    for qid in 0..q {
        let cmd = readv::<usize>();
        if cmd[0] == 1 {
            let x = cmd[1];
            *cnt.entry(x).or_insert(0) += 1;
        } else if cmd[0] == 2 {
            let x = cmd[1];
            let c = *cnt.get(&x).unwrap_or(&0);
            cnt.insert(x, c - 1);
            if c == 1 {
                cnt.remove(&x);
            }
        } else {
            println!("{}", cnt.len());
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
