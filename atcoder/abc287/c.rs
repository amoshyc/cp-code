#![allow(unused)]

use std::collections::HashMap;
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

    let (nodes, _, _) = bfs(&adj, 0);
    let s = nodes[nodes.len() - 1];
    let (nodes, parent, depth) = bfs(&adj, s);
    let t = nodes[nodes.len() - 1];
    let diameter = depth[t];

    // println!("{}", diameter);

    if diameter + 1 == n {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn bfs(adj: &Vec<Vec<usize>>, root: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut nodes = vec![];
    let mut parent = vec![std::usize::MAX; n];
    let mut depth = vec![std::usize::MAX; n];
    let mut queue = VecDeque::new();

    parent[root] = root;
    depth[root] = 0;

    queue.push_back(root);
    while let Some(u) = queue.pop_front() {
        nodes.push(u);
        for &v in adj[u].iter() {
            if parent[v] == std::usize::MAX {
                parent[v] = u;
                depth[v] = depth[u] + 1;
                queue.push_back(v);
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
