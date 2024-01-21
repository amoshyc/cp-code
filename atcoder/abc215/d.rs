#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let (n, m) = read2::<usize, usize>();
    let arr = readv::<usize>();
    let mut set: HashSet<usize> = HashSet::new();
    set.extend(arr.iter());
    
    let mut f = vec![true; m + 1];
    let max = m.max(*arr.iter().max().unwrap());
    for i in (2..=m) {
        let p = (i..=max).step_by(i).collect::<Vec<_>>();
        if p.iter().any(|x| set.contains(&x)) {
            for &x in p.iter() {
                if x <= m {
                    f[x] = false;
                }
            }
        }
    }

    let ans = (1..=m).filter(|&i| f[i]).collect::<Vec<_>>();
    println!("{}", ans.len());
    println!("{}", join(&ans, "\n"));
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read2<T: std::str::FromStr, F: std::str::FromStr>() -> (T, F) {
    let mut inp = read::<String>();
    let mut token = inp.split_ascii_whitespace();
    let a: T = token.next().unwrap().parse().ok().unwrap();
    let b: F = token.next().unwrap().parse().ok().unwrap();
    (a, b)
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
