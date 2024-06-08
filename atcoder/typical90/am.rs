#![allow(unused)]

// [Problem]
// You are given a tree with N vertices. Each vertex of the tree is numbered from 1 to N.
// The i-th edge connects vertices ai and bi bidirectionally, and all edges have a length of 1.
// Please calculate the value of sum(dist(u, v) for u in 1..(n - 1) for v in (u + 1)..n)
// where dist(u,v) represents the shortest distance from vertex u to vertex v

// [Solution]
// dist(u, v) is commutative therefore the answer is half of the pairwise total distance.
// It can be solved using rerooting.
// Let dp[u] = total distance starting from u,
// then dp[u] = dp[p] - A + B where
// p = the parent of u
// A = total decrement of distances to bottom nodes = siz[u]
// B = total increment of distances to top nodes = n - siz[u]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let edge = readv::<usize>();
        let (u, v) = (edge[0] - 1, edge[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut dep = vec![0; n];
    let mut siz = vec![0; n];
    dfs(0, !0, 0, &adj, &mut dep, &mut siz);

    let mut dp = vec![0; n];
    dp[0] = dep.iter().sum::<usize>();
    rec(0, !0, &adj, &siz, &mut dp);

    let ans = dp.iter().sum::<usize>() / 2;
    println!("{}", ans);
}

fn dfs(
    u: usize,
    p: usize,
    d: usize,
    adj: &Vec<Vec<usize>>,
    dep: &mut Vec<usize>,
    siz: &mut Vec<usize>,
) {
    dep[u] = d;
    siz[u] = 1;
    for v in adj[u].iter() {
        if *v != p {
            dfs(*v, u, d + 1, adj, dep, siz);
            siz[u] += siz[*v];
        }
    }
}

fn rec(u: usize, p: usize, adj: &Vec<Vec<usize>>, siz: &Vec<usize>, dp: &mut Vec<usize>) {
    if u > 0 {
        let a = siz[u];
        let b = adj.len() - siz[u];
        dp[u] = dp[p] + b - a;
    }
    for v in adj[u].iter() {
        if *v != p {
            rec(*v, u, adj, siz, dp);
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
