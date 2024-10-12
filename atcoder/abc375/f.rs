#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m, q) = (inp[0], inp[1], inp[2]);

    let inf = 10i64.pow(18);
    let mut adj = vec![vec![inf; n]; n];
    for u in 0..n {
        adj[u][u] = 0;
    }
    let mut edges = vec![];
    for _ in 0..m {
        let inp = readv::<i64>();
        let u = inp[0] as usize - 1;
        let v = inp[1] as usize - 1;
        let w = inp[2];
        adj[u][v] = w;
        adj[v][u] = w;
        edges.push((u, v, w));
    }

    let mut asks = vec![];
    for _ in 0..q {
        let ask = readv::<usize>();
        if ask[0] == 1 {
            let (u, v, w) = edges[ask[1] - 1];
            adj[u][v] = inf;
            adj[v][u] = inf;
            asks.push(('r', u, v, w)); // restore
        } else {
            let u = ask[1] - 1;
            let v = ask[2] - 1;
            asks.push(('a', u, v, 0));
        }
    }

    let mut dis = floyd_warshall(&adj, inf);
    let mut ans = vec![];
    for &(cmd, u, v, w) in asks.iter().rev() {
        if cmd == 'r' {
            if w >= dis[u][v] {
                continue;
            }
            dis[u][v] = w;
            dis[v][u] = w;
            for s in 0..n {
                for t in 0..n {
                    if dis[s][u] != inf && dis[v][t] != inf {
                        dis[s][t] = dis[s][t].min(dis[s][u] + dis[u][v] + dis[v][t]);
                    }
                    if dis[s][v] != inf && dis[u][t] != inf {
                        dis[s][t] = dis[s][t].min(dis[s][v] + dis[v][u] + dis[u][t]);
                    }
                }
            }
        } else {
            if dis[u][v] == inf {
                ans.push(-1);
            } else {
                ans.push(dis[u][v]);
            }
        }
    }
    ans.reverse();
    println!("{}", join(&ans, "\n"));
}

fn floyd_warshall(adj: &Vec<Vec<i64>>, inf: i64) -> Vec<Vec<i64>> {
    // dp[k][u][v] = minimum distance from u to v using vertices 0..=k as intermediate
    // dp[0][u][v] = adj[u][v]
    // dp[k][u][v] = min(dp[k - 1][u][v], dp[k - 1][u][k] + dp[k - 1][k][v]);
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
