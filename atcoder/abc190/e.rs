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

    let k = read::<usize>();
    let mut verts = vec![];
    for &v in readv::<usize>().iter() {
        verts.push(v - 1);
    }

    let inf = 10i64.pow(18);

    let mut pairwise = vec![vec![inf; k]; k];
    for i in 0..k {
        let mut que = std::collections::VecDeque::new();
        let mut dis = vec![inf; n];
        que.push_back(verts[i]);
        dis[verts[i]] = 0;
        while let Some(u) = que.pop_front() {
            for &v in adj[u].iter() {
                if dis[v] == inf {
                    dis[v] = dis[u] + 1;
                    que.push_back(v);
                }
            }
        }
        for j in 0..k {
            pairwise[i][j] = dis[verts[j]];
        }
    }

    // dp[S][u] = minimum total path if visited set = S while at u currently
    // dp[S][u] -> dp[S + {v}][v] if (u, v, w) is an edge
    let mut dp = vec![vec![inf; k]; (1 << k)];
    for u in 0..k {
        dp[(1 << u)][u] = 0;
    }
    for s in 0..(1 << k) {
        for u in 0..k {
            if (s >> u) & 1 != 0 {
                for v in 0..k {
                    if u != v && (s >> v) & 1 == 0 && pairwise[u][v] != inf {
                        dp[s | (1 << v)][v] = dp[s | (1 << v)][v].min(dp[s][u] + pairwise[u][v]);
                    }
                }
            }
        }
    }

    let ans = *dp[(1 << k) - 1].iter().min().unwrap();
    if ans == inf {
        println!("-1");
    } else {
        println!("{}", ans + 1);
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
