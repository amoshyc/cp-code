#![allow(unused)]

const MOD: u64 = 1_000_000_007;

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

    let inf = n + 10;
    let mut dis = vec![inf; n];
    let mut cnt = vec![0; n];
    let mut que = std::collections::VecDeque::new();
    dis[0] = 0;
    cnt[0] = 1;
    que.push_back(0);

    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if dis[u] + 1 < dis[v] {
                dis[v] = dis[u] + 1;
                cnt[v] = cnt[u];
                que.push_back(v);
            } else if dis[u] + 1 == dis[v] {
                cnt[v] += cnt[u];
                cnt[v] %= MOD;
            }
        }
    }

    println!("{}", cnt[n - 1]);
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
