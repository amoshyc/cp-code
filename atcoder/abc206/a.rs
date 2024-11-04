#![allow(unused)]

use std::cmp::Ordering;

fn main() {
    let x = read::<i64>();
    let y = x * 108 / 100;
    println!(
        "{}",
        match y.cmp(&206) {
            Ordering::Greater => ":(",
            Ordering::Equal => "so-so",
            Ordering::Less => "Yay!",
        }
    );
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
