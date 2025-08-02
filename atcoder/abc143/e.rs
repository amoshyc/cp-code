#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, l) = (inp[0], inp[1], inp[2] as i64);
    let inf = 10i64.pow(18);

    let mut adj = vec![vec![inf; n]; n];
    for u in 0..n {
        adj[u][u] = 0;
    }
    for _ in 0..m {
        let edge = readv::<usize>();
        let (u, v, w) = (edge[0] - 1, edge[1] - 1, edge[2] as i64);
        adj[u][v] = w;
        adj[v][u] = w;
    }
    let dis = floyd_warshall(&adj, inf);

    let mut shortcuts = vec![vec![inf; n]; n];
    for u in 0..n {
        shortcuts[u][u] = 0;
        for v in 0..n {
            if dis[u][v] <= l {
                shortcuts[u][v] = 1;
                shortcuts[v][u] = 1;
            }
        }
    }
    let jumps = floyd_warshall(&shortcuts, inf);

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        let (u, v) = (ask[0] - 1, ask[1] - 1);
        if jumps[u][v] == inf {
            ans.push(-1);
        } else {
            ans.push(jumps[u][v] - 1);
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn floyd_warshall(adj: &Vec<Vec<i64>>, inf: i64) -> Vec<Vec<i64>> {
    // dp[k][u][v] = minimum distance from u to v using vertices 0..=k as intermediate
    // dp[-1][u][v] = adj[u][v]
    // dp[k][u][v] = min(dp[k - 1][u][v], dp[k - 1][u][k] + dp[k - 1][k][v]);
    // adj[u][u] is usally 0. Remember to check it.
    let n = adj.len();
    let mut dp = adj.clone();
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                if dp[u][k] != inf && dp[k][v] != inf {
                    dp[u][v] = dp[u][v].min(dp[u][k] + dp[k][v]);
                }
            }
        }
    }
    dp
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
