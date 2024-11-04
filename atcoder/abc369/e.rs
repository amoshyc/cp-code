#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let inf = 1 << 60;
    let mut adj = vec![vec![inf; n]; n];
    let mut es = vec![];
    for u in 0..n {
        adj[u][u] = 0;
    }
    for _ in 0..m {
        let edge = readv::<i64>();
        let u = edge[0] as usize - 1;
        let v = edge[1] as usize - 1;
        let w = edge[2];
        es.push((u, v, w));
        adj[u][v] = adj[u][v].min(w);
        adj[v][u] = adj[v][u].min(w);
    }

    let dis = floyd_warshall(&adj, inf);

    let q = read::<usize>();
    let mut ans = vec![i64::MAX; q];
    for qid in 0..q {
        let k = read::<usize>();
        let b = readv::<usize>();
        let b = mapv(&b, |&x| x - 1);

        for perm in perm_iter(k) {
            for mask in 0..(1 << k) {
                let mut last = 0;
                let mut cost = 0;
                for i in 0..k {
                    let (mut u, mut v, w) = es[b[perm[i]]];
                    if (mask >> i) & 1 == 1 {
                        (u, v) = (v, u);
                    }
                    cost += dis[last][u]; // last -> u
                    cost += w; // u -> v
                    last = v;
                }
                cost += dis[last][n - 1];
                ans[qid] = ans[qid].min(cost);
            }
        }
    }

    println!("{}", join(&ans, "\n"));
}

fn floyd_warshall(adj: &Vec<Vec<i64>>, inf: i64) -> Vec<Vec<i64>> {
    // dp[k][u][v] = minimum distance from u to v using vertices 0..=k as intermediate
    // dp[-1][u][v] = adj[u][v]
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

fn next_perm<T: Ord>(arr: &mut [T]) -> Option<()> {
    let k = arr.windows(2).rposition(|w| w[0] < w[1])?;
    let j = arr.iter().rposition(|a| a > &arr[k]).unwrap();
    arr.swap(k, j);
    arr[(k + 1)..].reverse();
    Some(())
}

fn perm_iter(n: usize) -> impl std::iter::Iterator<Item = Vec<usize>> {
    let mut perm: Vec<usize> = (0..n).collect();
    let iter1 = std::iter::once(perm.clone());
    let iter2 = std::iter::from_fn(move || next_perm(&mut perm).and_then(|_| Some(perm.clone())));
    iter1.chain(iter2)
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
