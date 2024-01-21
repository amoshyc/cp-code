#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    let mut deg = vec![0; n];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
        deg[u] += 1;
        deg[v] += 1;
    }

    let mut ans = vec![];
    let mut que = std::collections::VecDeque::new();
    for u in 0..n {
        if deg[u] == 1 {
            que.push_back((u, 0));
        }
    }

    while let Some((u, p)) = que.pop_front() {
        if p == 1 {
            ans.push(adj[u].len());
        }
        for &v in adj[u].iter() {
            deg[v] -= 1;
            if deg[v] == 1 {
                que.push_back((v, (p + 1) % 3));
            }
        }
    }

    ans.sort();

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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
