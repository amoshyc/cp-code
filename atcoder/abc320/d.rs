#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<i64>();
        let (u, v) = (inp[0] as usize - 1, inp[1] as usize - 1);
        let (dx, dy) = (inp[2], inp[3]);
        adj[u].push((v, dx, dy));
        adj[v].push((u, -dx, -dy));
    }

    let inf = 10i64.pow(18);
    let mut que = std::collections::VecDeque::new();
    let mut x = vec![inf; n];
    let mut y = vec![inf; n];
    x[0] = 0;
    y[0] = 0;
    que.push_back(0);
    while let Some(u) = que.pop_front() {
        for &(v, dx, dy) in adj[u].iter() {
            if x[v] == inf {
                x[v] = x[u] + dx;
                y[v] = y[u] + dy;
                que.push_back(v);
            }
        }
    }

    let mut ans = vec![];
    for i in 0..n {
        if x[i] == inf {
            ans.push("undecidable".to_string());
        } else {
            ans.push(format!("{} {}", x[i], y[i]));
        }
    }

    println!("{}", join(&ans, "\n"));
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
