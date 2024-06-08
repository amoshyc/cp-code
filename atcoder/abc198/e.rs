#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let edge = readv::<usize>();
        let (u, v) = (edge[0] - 1, edge[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut cnt = vec![0; 100_001];
    let mut ans = vec![];
    cnt[arr[0]] += 1;
    dfs(0, usize::MAX, &mut cnt, &mut ans, &adj, &arr);
    ans.sort();
    println!("{}", join(&ans, "\n"));
}

fn dfs(
    u: usize,
    p: usize,
    cnt: &mut Vec<usize>,
    ans: &mut Vec<usize>,
    adj: &Vec<Vec<usize>>,
    arr: &Vec<usize>,
) {
    if cnt[arr[u]] == 1 {
        ans.push(u + 1);
    }
    for &v in adj[u].iter() {
        if v != p {
            cnt[arr[v]] += 1;
            dfs(v, u, cnt, ans, adj, arr);
            cnt[arr[v]] -= 1;
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
