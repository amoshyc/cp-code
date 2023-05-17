use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let inp = readv::<usize>();
    let (n, m, x, y) = (inp[0], inp[1], inp[2] - 1, inp[3] - 1);

    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (a, b, t, k) = (inp[0] - 1, inp[1] - 1, inp[2], inp[3]);
        adj[a].push((b, t as i64, k as i64));
        adj[b].push((a, t as i64, k as i64));
    }

    let mut que = BinaryHeap::new();
    let mut dis = vec![std::i64::MAX; n];
    dis[x] = 0;
    que.push(Reverse((dis[x], x)));

    while !que.is_empty() {
        let Reverse((d, u)) = que.pop().unwrap();
        if d > dis[u] {
            continue;
        }
        for &(v, t, k) in adj[u].iter() {
            let new_d = (d + (k - 1)) / k * k + t;
            if new_d < dis[v] {
                dis[v] = new_d;
                que.push(Reverse((dis[v], v)));
            }
        }
    }

    if dis[y] == std::i64::MAX {
        println!("-1");
    } else {
        println!("{}", dis[y]);
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
