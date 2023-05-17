#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let n = read::<usize>();
    let s = reads();
    let mut x = 0;
    let mut y = 0;
    let mut v = HashSet::new();
    let mut ans = false;
    v.insert((0, 0));
    for &c in s.iter() {
        match c {
            'L' => x -= 1,
            'R' => x += 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => (),
        }
        if v.contains(&(x, y)) {
            ans = true;
        } else {
            v.insert((x, y));
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
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
