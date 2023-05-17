#![allow(unused)]

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

    let mut ans = 0;
    let mut vis = vec![false; n];
    vis[0] = true;
    dfs(0, &adj, &mut vis, &mut ans);
    println!("{}", ans);
}

fn dfs(u: usize, adj: &[Vec<usize>], vis: &mut [bool], ans: &mut usize) {
    if *ans >= 1_000_000 {
        return;
    } else {
        *ans += 1;
    }

    for &v in adj[u].iter() {
        if !vis[v] {
            vis[v] = true;
            dfs(v, adj, vis, ans);
            vis[v] = false;
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
