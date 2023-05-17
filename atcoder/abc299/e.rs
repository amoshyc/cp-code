#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }
    let k = read::<usize>();
    let mut p = vec![];
    let mut d = vec![];
    for _ in 0..k {
        let inp = readv::<usize>();
        p.push(inp[0] - 1);
        d.push(inp[1]);
    }

    let mut must_white = vec![false; n];
    let mut must_black = vec![false; n];

    for i in 0..k {
        if d[i] == 0 {
            must_black[p[i]] = true;
            continue;
        }
        let (path, dis) = bfs(&adj, p[i], 1_000_000_000);
        for j in 0..n {
            if dis[j] < d[i] {
                must_white[j] = true;
            }
        }
    }

    let mut ok = true;
    let mut color = vec![1; n];
    for i in 0..n {
        if must_black[i] && must_white[i] {
            ok = false;
        }
        if must_white[i] {
            color[i] = 0;
        } else {
            color[i] = 1;
        }
    }
    ok = ok & color.iter().any(|&c| c == 1);

    for i in 0..k {
        let (_, dis) = bfs(&adj, p[i], 1_000_000_000);
        ok = ok & (0..n).filter(|&j| dis[j] == d[i]).any(|j| color[j] == 1);
    }

    if ok {
        println!("Yes");
        println!("{}", join(&color, ""));
    } else {
        println!("No");
    }
}

fn bfs(adj: &Vec<Vec<usize>>, root:usize, inf: usize) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut que = VecDeque::new();
    let mut dis = vec![inf; n];
    let mut path = vec![];
    dis[root] = 0;
    que.push_back(root);
    while let Some(u) = que.pop_front() {
        path.push(u);
        for &v in adj[u].iter() {
            if dis[v] == inf {
                dis[v] = dis[u] + 1;
                que.push_back(v);
            }
        }
    }
    (path, dis)
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
