#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);
    let (s, t, x) = (inp[3] - 1, inp[4] - 1, inp[5] - 1);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    // dp[i, u, j] = number of valid A[..=i] while A[i] = u and number of x % 2 = j
    let mut dp = vec![vec![[0, 0]; n + 1]; k + 1];
    dp[0][s][0] = 1;
    for i in 0..k {
        for u in 0..n {
            for j in 0..2 {
                for &v in adj[u].iter() {
                    // dp[i, u, j] -> dp[i + 1, v, j']
                    let new_j = (j + if v == x { 1 } else { 0 }) % 2;
                    dp[i + 1][v][new_j] += dp[i][u][j];
                    dp[i + 1][v][new_j] %= 998244353;
                }
            }
        }
    }

    println!("{}", dp[k][t][0]);
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
