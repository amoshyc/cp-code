#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut adj = vec![vec![]; n];
    let mut inv = vec![vec![]; n];
    for i in 0..n {
        let s = reads();
        for j in 0..m {
            if s[j] == '1' {
                adj[i].push(i + j + 1);
                inv[i + j + 1].push(i);
            }
        }
    }

    let inf = 10usize.pow(8);
    let dis1 = bfs(&adj, 0, inf);
    let dis2 = bfs(&inv, n - 1, inf);

    let mut ans = vec![!0; n];
    for u in 0..n {
        for &v in adj[u].iter() {
            for x in (u.min(v) + 1)..(u.max(v)) {
                ans[x] = ans[x].min(dis1[u] + dis2[v] + 1);
            }
        }
    }

    let ans = ans
        .iter()
        .map(|&d| {
            if d < inf {
                d.to_string()
            } else {
                "-1".to_string()
            }
        })
        .collect::<Vec<String>>();
    println!("{}", join(&ans[1..(n - 1)], " "));
}

fn bfs(adj: &Vec<Vec<usize>>, root: usize, inf: usize) -> Vec<usize> {
    let n = adj.len();
    let mut dis = vec![inf; n];
    let mut que = std::collections::VecDeque::new();
    dis[root] = 0;
    que.push_back(root);
    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if dis[u] + 1 < dis[v] {
                dis[v] = dis[u] + 1;
                que.push_back(v);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
