#![allow(unused)]

// [Problem]
// There are N cities, each numbered from 1 to N.
// Additionally, there are N−1 roads, where the i-th road (1≤i≤N−1) connects cities Ai and Bi bidirectionally.
// It is possible to travel between any pair of cities via some roads.
// Now, you are free to choose integers u and v (1≤u<v≤N) and construct exactly one new bidirectional road between city u and city v.
// Define the score as follows:
// The number of roads traveled in a path from a city to the same city without passing through the same road twice (this value is uniquely determined).
// Please output the maximum possible value that can be considered as the score.
// It is possible to travel between any pair of cities via some roads.

// [Solution]
// The problem is asking the size of the largest cycle that can formed by adding an edge.
// It is the longest path in current graph plus the added edge.
// The longest path in the graph is the diameter of the graph.
// It is well known to use two BFS to find the diameter.

use std::collections::VecDeque;

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let dep1 = bfs(0, &adj);
    let u = (0..n).max_by_key(|u| dep1[*u]).unwrap();
    let dep2 = bfs(u, &adj);
    let diameter = dep2.iter().max().unwrap();
    println!("{}", diameter + 1);
}

fn bfs(root: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = adj.len();
    let mut que = VecDeque::new();
    let mut dep = vec![usize::MAX; n];
    que.push_back(root);
    dep[root] = 0;
    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if dep[v] == usize::MAX {
                dep[v] = dep[u] + 1;
                que.push_back(v);
            }
        }
    }
    dep
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
