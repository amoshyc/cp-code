#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut adj = vec![vec![]; n + m];
    for i in 0..n {
        let l = read::<usize>();
        let arr = readv::<usize>();
        for j in 0..l {
            let u = m + i;
            let v = arr[j] - 1;
            adj[u].push((v, 0));
            adj[v].push((u, 1));
        }
    }

    let inf = (10 as i64).pow(18);
    let mut que = std::collections::VecDeque::new();
    let mut dis = vec![inf; n + m];
    dis[0] = 0;
    que.push_back(0);

    // 01 bfs
    while let Some(u) = que.pop_front() {
        for &(v, w) in adj[u].iter() {
            let new_d = dis[u] + w;
            if new_d < dis[v] {
                dis[v] = new_d;
                if w == 0 {
                    que.push_front(v);
                } else {
                    que.push_back(v);
                }
            }
        }
    }

    if dis[m - 1] == inf {
        println!("-1");
    } else {
        println!("{}", dis[m - 1] - 1);
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

fn reads() -> Vec<char> {
    read::<String>().chars().collect::<Vec<char>>()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
