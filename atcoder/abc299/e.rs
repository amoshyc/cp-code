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

    let mut color = vec![1; n];
    for i in 0..k {
        let (path, _, dis) = bfs(&adj, vec![p[i]]);
        for j in 0..n {
            if dis[j] < d[i] {
                color[j] = 0;
            }
        }
    }

    let mut ok = color.iter().any(|&c| c == 1);
    for i in 0..k {
        let (_, _, dis) = bfs(&adj, vec![p[i]]);
        ok = ok & (0..n).filter(|&j| dis[j] == d[i]).any(|j| color[j] == 1);
    }

    if ok {
        println!("Yes");
        println!("{}", join(&color, ""));
    } else {
        println!("No");
    }
}

fn bfs(adj: &Vec<Vec<usize>>, mut roots: Vec<usize>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let inf = !0;
    let mut nodes = vec![];
    let mut parent = vec![inf; n];
    let mut depth = vec![inf; n];
    let mut queue = VecDeque::new();

    if roots.len() == 0 {
        roots.extend(0..n);
    }

    for root in roots {
        if parent[root] == inf {
            parent[root] = root;
            depth[root] = 0;
            queue.push_back(root);
            while let Some(u) = queue.pop_front() {
                nodes.push(u);
                for &v in adj[u].iter() {
                    if parent[v] == inf {
                        parent[v] = u;
                        depth[v] = depth[u] + 1;
                        queue.push_back(v);
                    }
                }
            }
        }
    }

    (nodes, parent, depth)
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
