#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    let mut es = vec![];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
        es.push((u, v));
    }

    let mut dep = vec![!0; n];
    let mut que = std::collections::VecDeque::new();
    dep[0] = 0;
    que.push_back(0);
    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if dep[v] == !0 {
                dep[v] = dep[u] + 1;
                que.push_back(v);
            }
        }
    }

    let mut diff = vec![0i64; n];
    let q = read::<usize>();
    for _ in 0..q {
        let inp = readv::<i64>();
        let (t, i, x) = (inp[0], inp[1] as usize, inp[2]);
        let (mut u, mut v) = es[i - 1];
        if t == 1 {
            if dep[u] < dep[v] {
                diff[0] += x;
                diff[v] -= x;
            } else {
                diff[u] += x;
            }
        } else {
            if dep[u] < dep[v] {
                diff[v] += x;
            } else {
                diff[0] += x;
                diff[u] -= x;
            }
        }
    }

    let mut ans = vec![0; n];
    let mut vis = vec![false; n];
    let mut que = std::collections::VecDeque::new();
    ans[0] = diff[0];
    vis[0] = true;
    que.push_back(0);
    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if !vis[v] {
                vis[v] = true;
                ans[v] = ans[u] + diff[v];
                que.push_back(v);
            }
        }
    }

    println!("{}", join(&ans, "\n"));
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
