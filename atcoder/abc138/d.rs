#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut que = std::collections::VecDeque::new();
    let mut vis = vec![false; n];
    let mut par = vec![n; n];
    let mut ord = vec![];
    que.push_back(0);
    vis[0] = true;
    while let Some(u) = que.pop_front() {
        ord.push(u);
        for v in adj[u].iter() {
            if !vis[*v] {
                vis[*v] = true;
                par[*v] = u;
                que.push_back(*v);
            }
        }
    }

    let mut cnt = vec![0 as i64; n];
    for _ in 0..q {
        let inp = readv::<usize>();
        let (p, x) = (inp[0] - 1, inp[1] as i64);
        cnt[p] += x;
    }

    for u in ord {
        if par[u] != n {
            cnt[u] += cnt[par[u]];
        }
    }

    println!("{}", join(&cnt, " "));
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
