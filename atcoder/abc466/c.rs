#![allow(unused)]

use std::io::Write;

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    let mut out = std::io::stdout();
    let mut ask = |i, j| -> bool {
        println!("? {} {}", i + 1, j + 1);
        out.flush();
        let ok = reads();
        ok[0] == 'Y'
    };

    let n = read::<usize>();

    let mut ans = 0;
    let mut j = 1;
    for i in 0..n {
        j = j.max(i + 1);
        while j < n && ask(i, j) {
            j += 1;
        }

        ans += (j - i - 1).max(0) as i64;
    }

    println!("! {ans}");
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
