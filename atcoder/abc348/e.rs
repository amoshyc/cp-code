#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let c = readv::<i64>();
    let mut dep = vec![0; n];
    let mut sum = vec![0i64; n]; // sum of c of the subtree
    dfs(0, !0, 0, &mut dep, &mut sum, &adj, &c);

    let total = c.iter().sum::<i64>();
    let mut dp = vec![0i64; n];
    for v in 0..n {
        dp[0] += dep[v] as i64 * c[v];
    }
    rec(0, !0, &mut dp, &sum, &adj, &c, total);

    println!("{}", dp.iter().min().unwrap());
}

fn dfs(
    u: usize,
    p: usize,
    d: usize,
    dep: &mut Vec<usize>,
    sum: &mut Vec<i64>,
    adj: &Vec<Vec<usize>>,
    c: &Vec<i64>,
) {
    dep[u] = d;
    sum[u] = c[u];
    for v in adj[u].iter() {
        if *v != p {
            dfs(*v, u, d + 1, dep, sum, adj, c);
            sum[u] += sum[*v];
        }
    }
}

fn rec(
    u: usize,
    p: usize,
    dp: &mut Vec<i64>,
    sum: &Vec<i64>,
    adj: &Vec<Vec<usize>>,
    c: &Vec<i64>,
    total: i64,
) {
    if u > 0 {
        dp[u] = dp[p];
        dp[u] -= sum[u] + c[u]; // bottom
        dp[u] += total - sum[u] + c[u]; // top
    }
    for v in adj[u].iter() {
        if *v != p {
            rec(*v, u, dp, sum, adj, c, total);
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
