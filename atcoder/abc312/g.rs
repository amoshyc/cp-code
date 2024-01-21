#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut adj = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let (a, b) = read2::<usize, usize>();
        let (a, b) = (a - 1, b - 1);
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut siz = vec![0i64; n];
    let mut dis = 0;
    dfs(0, !0, &adj, &mut siz, &mut dis);

    let n = n as i64;
    let mut ans = 0;
    ans += n * (n - 1) * (n - 2) / 6;
    ans -= dis;
    ans += n * (n - 1) / 2;

    println!("{}", ans);
}

fn dfs(u: usize, p: usize, adj: &Vec<Vec<usize>>, siz: &mut Vec<i64>, dis: &mut i64) {
    siz[u] = 1;
    for &v in adj[u].iter() {
        if v != p {
            dfs(v, u, adj, siz, dis);
            siz[u] += siz[v];
            *dis += siz[v] * (adj.len() as i64 - siz[v]);
        }
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read2<T: std::str::FromStr, F: std::str::FromStr>() -> (T, F) {
    let mut inp = read::<String>();
    let mut token = inp.split_ascii_whitespace();
    let a: T = token.next().unwrap().parse().ok().unwrap();
    let b: F = token.next().unwrap().parse().ok().unwrap();
    (a, b)
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
