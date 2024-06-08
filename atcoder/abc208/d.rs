#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let inf = 10i64.pow(18);
    let mut adj = vec![vec![inf; n]; n];
    for u in 0..n {
        adj[u][u] = 0;
    }
    for _ in 0..m {
        let inp = readv::<i64>();
        let u = inp[0] as usize - 1;
        let v = inp[1] as usize - 1;
        let w = inp[2];
        adj[u][v] = w;
    }

    // floyd warshall
    // dp[k][u][v] = min distance from u to v using 0..=k
    let mut ans = 0;
    let mut dp = vec![vec![inf; n]; n];
    for u in 0..n {
        for v in 0..n {
            dp[u][v] = adj[u][v];
        }
    }

    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                dp[u][v] = dp[u][v].min(dp[u][k] + dp[k][v]);
                if dp[u][v] < inf {
                    ans += dp[u][v];
                }
            }
        }
    }

    println!("{}", ans);
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
    read::<String>().chars().collect::<_>()
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
