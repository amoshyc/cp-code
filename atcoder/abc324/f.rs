#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let inp = readv::<usize>();
        let (u, v) = (inp[0] - 1, inp[1] - 1);
        let (b, c) = (inp[2] as f64, inp[3] as f64);
        adj[u].push((v, b, c));
    }

    let inf: f64 = 1e18;
    let ok = |x: f64| {
        let mut dis = vec![-inf; n];
        dis[0] = 0.0;
        for u in 0..n {
            for &(v, b, c) in adj[u].iter() {
                let w = b - x * c;
                dis[v] = dis[v].max(dis[u] + w);
            }
        }
        dis[n - 1] >= 0.0
    };

    // 1 1 1 0 0 0
    let mut lb = 0.0;
    let mut ub = 1e10;
    for _ in 0..100 {
        let x = (lb + ub) / 2.0;
        if ok(x) {
            lb = x;
        } else {
            ub = x;
        }
    }
    println!("{:.10}", ub);
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
