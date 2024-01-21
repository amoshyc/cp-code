#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n1, n2, m) = (inp[0], inp[1], inp[2]);
    let mut adj1 = vec![vec![]; n1];
    let mut adj2 = vec![vec![]; n2];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        if inp[0] < n1 {
            adj1[u].push(v);
            adj1[v].push(u);
        } else {
            adj2[u - n1].push(v - n1);
            adj2[v - n1].push(u - n1);
        }
    }

    let d1 = bfs(&adj1, 0);
    let d2 = bfs(&adj2, n2 - 1);
    println!("{}", d1 + d2 + 1);
}

fn bfs(adj: &Vec<Vec<usize>>, root: usize) -> usize {
    let n = adj.len();
    let inf = !0;
    let mut dis = vec![inf; n];
    let mut que = std::collections::VecDeque::new();
    dis[root] = 0;
    que.push_back(root);
    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if dis[v] == inf {
                dis[v] = dis[u] + 1;
                que.push_back(v);
            }
        }
    }
    *dis.iter().max().unwrap()
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
