#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, q) = (inp[0], inp[1]);

    let mut adj = vec![std::collections::HashSet::new(); n];
    let mut cnt = n;
    let mut ans = vec![];
    for _ in 0..q {
        let inp = readv::<usize>();
        if inp[0] == 1 {
            let (u, v) = (inp[1] - 1, inp[2] - 1);
            if adj[u].len() == 0 {
                cnt -= 1;
            }
            if adj[v].len() == 0 {
                cnt -= 1;
            }
            adj[u].insert(v);
            adj[v].insert(u);
        } else {
            let v = inp[1] - 1;
            let neighbors = adj[v].clone();
            for &u in neighbors.iter() {
                adj[u].remove(&v);
                if adj[u].len() == 0 {
                    cnt += 1;
                }
            }
            if adj[v].len() > 0 {
                cnt += 1;
                adj[v].clear();
            }
        }
        ans.push(cnt);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
