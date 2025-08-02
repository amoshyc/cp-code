#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let inf = 10i64.pow(18);
    let mut adj = vec![vec![inf; n + 1]; n + 1];
    for u in 0..=n {
        adj[u][u] = 0;
    }
    for _ in 0..m {
        let inp = readv::<i64>();
        let u = inp[0] as usize - 1;
        let v = inp[1] as usize - 1;
        let w = inp[2];
        adj[u][v] = adj[u][v].min(w);
        adj[v][u] = adj[v][u].min(w);
    }

    let inp = readv::<i64>();
    let (_, t) = (inp[0] as usize, inp[1]);
    let arr = readv::<usize>();
    for &x in &arr {
        adj[x - 1][n] = t; // x -> sky with t
        adj[n][x - 1] = 0; // sky -> x with 0
    }

    let mut dis = floyd_warshall(&adj, inf);
    let relax = |dis: &mut Vec<Vec<i64>>, x: usize, y: usize, w: i64| {
        for i in 0..=n {
            for j in 0..=n {
                if dis[i][x] != inf && dis[y][j] != inf {
                    dis[i][j] = dis[i][j].min(dis[i][x] + w + dis[y][j]);
                }
            }
        }
    };

    let q = read::<usize>();
    let mut ans = vec![];
    for _ in 0..q {
        let ask = readv::<i64>();
        if ask[0] == 1 {
            let x = ask[1] as usize - 1;
            let y = ask[2] as usize - 1;
            let w = ask[3];
            relax(&mut dis, x, y, w); // x -> y
            relax(&mut dis, y, x, w); // y -> x
        } else if ask[0] == 2 {
            let x = ask[1] as usize - 1;
            relax(&mut dis, x, n, t); // x -> sky
            relax(&mut dis, n, x, 0); // sky -> x
        } else {
            let mut val = 0;
            for u in 0..n {
                for v in 0..n {
                    if dis[u][v] != inf {
                        val += dis[u][v];
                    }
                }
            }
            ans.push(val);
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
