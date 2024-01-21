#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    let mut inv = vec![vec![]; n];
    let mut ind = vec![0; n];
    for u in 0..n {
        let inp = readv::<usize>();
        for &v in inp[1..].iter() {
            let v = v - 1;
            adj[u].push(v);
            inv[v].push(u);
            ind[u] += 1;
        }
    }

    let mut vis = vec![false; n];
    let mut que = std::collections::VecDeque::new();
    que.push_back(0);
    vis[0] = true;
    while let Some(u) = que.pop_front() {
        for &v in adj[u].iter() {
            if !vis[v] {
                vis[v] = true;
                que.push_back(v);
            }
        }
    }

    let mut que = std::collections::VecDeque::new();
    for u in 0..n {
        if vis[u] && ind[u] == 0 {
            que.push_back(u);
        }
    }

    let mut ans = vec![];
    while let Some(u) = que.pop_front() {
        ans.push(u + 1);
        for &v in inv[u].iter() {
            ind[v] -= 1;
            if vis[v] && ind[v] == 0 {
                que.push_back(v);
            }
        }
    }

    if *ans.last().unwrap() == 1 {
        ans.pop();
    }

    println!("{}", join(&ans, " "));
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
