#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let edge = readv::<u64>();
        let u = edge[0] as usize - 1;
        let v = edge[1] as usize - 1;
        let w = edge[2];
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut ans = u64::MAX;
    let mut vis = vec![false; n];
    vis[0] = true;
    dfs(0, 0, &mut vis, &mut ans, &adj);
    println!("{}", ans);
}

fn dfs(u: usize, xor: u64, vis: &mut Vec<bool>, ans: &mut u64, adj: &Vec<Vec<(usize, u64)>>) {
    let n = adj.len();
    if u == n - 1 {
        *ans = (*ans).min(xor);
        return;
    }
    for &(v, w) in &adj[u] {
        if !vis[v] {
            vis[v] = true;
            dfs(v, xor ^ w, vis, ans, adj);
            vis[v] = false;
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
