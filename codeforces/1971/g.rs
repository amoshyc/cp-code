#![allow(unused)]

use std::collections::{HashMap, VecDeque};

fn main() {
    let tc = read::<usize>();

    for _ in 0..tc {
        let n = read::<usize>();
        let arr = readv::<usize>();

        let mut inv = HashMap::new();
        for i in 0..n {
            inv.entry(arr[i]).or_insert(vec![]).push(i);
        }

        let mut vis = vec![false; n];
        let mut ans = vec![0; n];

        for r in 0..n {
            if vis[r] {
                continue;
            }

            let mut group = vec![];
            for i in inv[&arr[r]].iter() {
                if !vis[*i] {
                    vis[*i] = true;
                    group.push(*i);
                }
            }
            inv.remove(&arr[r]);

            let mut que = VecDeque::new();
            que.push_back(arr[r]);
            while let Some(u) = que.pop_front() {
                for v in [u ^ 1, u ^ 2, u ^ 3] {
                    if let Some(pos) = inv.get(&v) {
                        for i in pos {
                            if !vis[*i] {
                                vis[*i] = true;
                                group.push(*i);
                                que.push_back(v);
                            }
                        }
                        inv.remove(&v);
                    }
                }
            }

            group.sort();
            let mut indices = group.clone();
            indices.sort_by_key(|i| arr[*i]);
            for (i, j) in group.iter().zip(indices.iter()) {
                ans[*i] = arr[*j];
            }
        }

        println!("{}", join(&ans, " "));
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
    read::<String>().chars().collect::<_>()
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
