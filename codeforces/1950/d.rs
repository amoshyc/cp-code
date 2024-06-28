#![allow(unused)]

use std::collections::HashSet;

fn main() {
    let mut b = vec![];
    for i in 0..(1 << 6) {
        let x = format!("{:b}", i).parse::<usize>().unwrap();
        if x <= 100_000 {
            b.push(x);
        }
    }
    b.sort();
    b.dedup();

    let mut set = HashSet::new();
    dfs(1, &mut set, &b);

    let tc = read::<usize>();
    let mut ans = vec![];
    for _ in 0..tc {
        let n = read::<usize>();
        if set.contains(&n) {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn dfs(x: usize, set: &mut HashSet<usize>, b: &Vec<usize>) {
    if x > 100_000 {
        return;
    } else {
        set.insert(x);
    }
    for &y in b.iter() {
        if y > 1 {
            dfs(x * y, set, b);
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
