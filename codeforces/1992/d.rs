#![allow(unused)]

use std::collections::VecDeque;

fn solve() {
    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);
    let mut arr = reads();
    arr.insert(0, 'L');
    arr.push('L');

    let mut adj = vec![vec![]; n + 2];
    for u in 0..(n + 2) {
        if arr[u] == 'C' {
            continue;
        } else if arr[u] == 'W' {
            let v = u + 1;
            if v < n + 2 && arr[v] != 'C' {
                let w = if arr[v] == 'W' { 1 } else { 0 };
                adj[u].push((v, w));
            }
        } else {
            for i in 0..m {
                let v = u + i + 1;
                if v < n + 2 && arr[v] != 'C' {
                    let w = if arr[v] == 'W' { 1 } else { 0 };
                    adj[u].push((v, w));
                }
            }
        }
    }

    // 01-BFS
    let inf = n + 10;
    let mut dis = vec![inf; n + 2];
    let mut que = VecDeque::new();

    dis[0] = 0;
    que.push_back(0);

    while let Some(u) = que.pop_front() {
        if u == n + 1 {
            break;
        }

        for &(v, w) in adj[u].iter() {
            if dis[v] == inf {
                dis[v] = dis[u] + w;
                if w == 0 {
                    que.push_front(v);
                } else {
                    que.push_back(v);
                }
            }
        }
    }

    if dis[n + 1] <= k {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn main() {
    let tc = read::<usize>();
    // let mut stdout = std::io::stdout();
    // let mut writer = std::io::BufWriter::new(stdout);
    for _ in 0..tc {
        solve();
    }
}

// use std::io::{BufWriter, Stdout, Write};
// type Writer = BufWriter<Stdout>;

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
