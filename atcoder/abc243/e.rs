#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut edges = vec![];
    for _ in 0..m {
        let inp = readv::<i64>();
        let (u, v) = (inp[0] as usize - 1, inp[1] as usize - 1);
        let w = inp[2];
        edges.push((u, v, w));
    }

    let inf = 10i64.pow(18);
    let mut dp = vec![vec![inf; n]; n];
    for u in 0..n {
        dp[u][u] = 0;
    }
    for &(u, v, w) in edges.iter() {
        dp[u][v] = w;
        dp[v][u] = w;
    }
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                dp[u][v] = dp[u][v].min(dp[u][k] + dp[k][v]);
            }
        }
    }

    let mut ans = 0;
    for &(u, v, w) in edges.iter() {
        if (0..n).filter(|&x| x != u && x != v).any(|x| dp[u][x] + dp[x][v] <= w) {
            ans += 1;
        }
    }
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
