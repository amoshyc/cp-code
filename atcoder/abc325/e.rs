#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let n = inp[0] as usize;
    let (a, b, c) = (inp[1], inp[2], inp[3]);

    let mut adj = vec![vec![]; n];
    let mut rev = vec![vec![]; n];
    for u in 0..n {
        let inp = readv::<i64>();
        for v in 0..n {
            let w = inp[v];
            adj[u].push((v, w * a));
            rev[v].push((u, w * b + c));
        }
    }

    let dis1 = dijkstra(&adj, 0);
    let dis2 = dijkstra(&rev, n - 1);

    let mut ans = 10i64.pow(18);
    for u in 0..n {
        ans = ans.min(dis1[u] + dis2[u]);
    }

    println!("{}", ans);
}

fn dijkstra(adj: &Vec<Vec<(usize, i64)>>, s: usize) -> Vec<i64> {
    let n = adj.len();
    let mut que = std::collections::BinaryHeap::new(); // max heap
    let mut dis = vec![10i64.pow(18); n];

    dis[s] = 0;
    que.push((std::cmp::Reverse(dis[s]), s));

    while let Some((std::cmp::Reverse(d), u)) = que.pop() {
        if d > dis[u] {
            continue;
        }
        for &(v, w) in adj[u].iter() {
            let new_d = dis[u] + w;
            if new_d < dis[v] {
                dis[v] = new_d;
                que.push((std::cmp::Reverse(dis[v]), v));
            }
        }
    }

    dis
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
