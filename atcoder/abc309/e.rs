#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    let p = readv::<usize>();
    for i in 0..(n - 1) {
        let u = i + 1;
        let v = p[i] - 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut mark = vec![0; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let x = inp[0] - 1;
        let y = inp[1];
        mark[x] = mark[x].max(y + 1);
    }

    let mut que = std::collections::VecDeque::new();
    let mut vis = vec![false; n];
    que.push_back(0);
    vis[0] = true;

    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if !vis[v] {
                vis[v] = true;
                que.push_back(v);
                if mark[u] > 0 {
                    if mark[u] - 1 > mark[v] {
                        mark[v] = mark[u] - 1;
                    }
                }
            }
        }
    }

    let ans = mark.iter().filter(|m| **m > 0).count();
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
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
