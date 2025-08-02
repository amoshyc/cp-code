#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let tc = read::<usize>();
    let mut ans = vec![];

    for i in 0..tc {
        let n = read::<usize>();
        let mut s = reads();
        s.insert(0, '0');

        let mut que = VecDeque::new();
        let mut vis = vec![false; 1 << n];
        vis[0] = true;
        que.push_back(0);
        while let Some(u) = que.pop_front() {
            for i in 0..n {
                if (u >> i) & 1 == 0 {
                    let v = u ^ (1 << i);
                    if !vis[v] && s[v] == '0' {
                        vis[v] = true;
                        que.push_back(v);
                    }
                }
            }
        }

        if vis[(1 << n) - 1] {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }

    println!("{}", join(&ans, "\n"));
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
