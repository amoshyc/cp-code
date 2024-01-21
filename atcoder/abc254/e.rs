#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut ans = vec![];
    let q = read::<usize>();
    for _ in 0..q {
        let inp = readv::<usize>();
        let (x, k) = (inp[0] - 1, inp[1]);

        let mut cnt = 0;
        let mut que = std::collections::VecDeque::new();
        let mut vis = std::collections::HashSet::new();
        que.push_back((x, 0));
        vis.insert(x);
        while let Some((u, d)) = que.pop_front() {
            cnt += (u + 1);
            for &v in adj[u].iter() {
                if !vis.contains(&v) && d < k {
                    vis.insert(v);
                    que.push_back((v, d + 1));
                }
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
