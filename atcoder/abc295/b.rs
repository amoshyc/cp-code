#![allow(unused)]

use std::{convert::TryFrom, collections::HashSet};

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for r in 0..n {
        arr.push(reads());
    }

    let mut bombs = vec![];
    for r in 0..n {
        for c in 0..m {
            if arr[r][c] != '.' && arr[r][c] != '#' {
                let x = arr[r][c].to_digit(10).unwrap();
                bombs.push((r, c, x));
            }
        }
    }

    let dis = |a: usize, b: usize, r: usize, c: usize| {
        let dr = ((a as i32) - (r as i32)).abs();
        let dc = ((b as i32) - (c as i32)).abs();
        (dr + dc) as u32
    };

    let mut res = vec![vec!['.'; m]; n];
    for r in 0..n {
        for c in 0..m {
            if bombs.iter().any(|&(a, b, x)| dis(a, b, r, c) <= x) {
                res[r][c] = '.';
            } else {
                res[r][c] = arr[r][c];
            }
        }
    }

    for r in 0..n {
        println!("{}", join(&res[r], ""));
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
