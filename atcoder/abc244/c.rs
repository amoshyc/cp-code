#![allow(unused)]

use std::io::Write;

fn main() {
    let n = read::<usize>();

    let mut set = std::collections::HashSet::new();
    set.extend((1..=(2 * n + 1)));

    while !set.is_empty() {
        let x = set.iter().next().unwrap().clone();
        println!("{}", x);
        std::io::stdout().flush();

        let y = read::<usize>();
        set.remove(&x);
        set.remove(&y);
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
