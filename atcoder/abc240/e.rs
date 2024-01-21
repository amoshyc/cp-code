#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut cnt = 0;
    let mut ans = vec![(0, 0); n];
    dfs(0, n, &adj, &mut ans, &mut cnt);

    let out: Vec<_> = ans
        .iter()
        .map(|&(l, r)| format!("{} {}", l + 1, r + 1))
        .collect();
    println!("{}", join(&out, "\n"));
}

fn dfs(u: usize, p: usize, adj: &Vec<Vec<usize>>, ans: &mut Vec<(usize, usize)>, cnt: &mut usize) {
    let mut min = adj.len();
    let mut max = 0;
    for &v in adj[u].iter() {
        if v != p {
            dfs(v, u, adj, ans, cnt);
            min = min.min(ans[v].0);
            min = min.min(ans[v].1);
            max = max.max(ans[v].0);
            max = max.max(ans[v].1);
        }
    }

    if min == adj.len() && max == 0 {
        ans[u] = (*cnt, *cnt);
        *cnt += 1;
    } else {
        ans[u] = (min, max);
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
