#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut sets = vec![HashSet::new(); m];
    for i in 0..m {
        let _ = read::<usize>();
        let arr = readv::<usize>();
        for x in arr {
            sets[i].insert(x);
        }
    }

    let mut ans = 0;
    for mask in 1..(1 << m) {
        let mut union: HashSet<usize> = HashSet::new();
        for i in 0..m {
            if (mask >> i) & 1 != 0 {
                union.extend(&sets[i]);
            }
        }
        if union.len() == n {
            ans += 1;
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
