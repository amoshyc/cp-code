#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut w = vec![];
    let mut v = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        w.push(inp[0]);
        v.push(inp[1]);
    }

    // dp[i][j] = min total weight when max value is j while using items 0..=i
    // dp[i][j] = min(dp[i - 1][j - v[i]] + w[i], dp[i - 1][j])
    let max_total_v = v.iter().max().unwrap() * n;
    let mut dp = vec![vec![m + 2; max_total_v + 1]; n];
    dp[0][0] = 0;
    dp[0][v[0]] = w[0];
    for i in 1..n {
        for j in 0..=max_total_v {
            dp[i][j] = dp[i][j].min(dp[i - 1][j]);
            if j >= v[i] {
                dp[i][j] = dp[i][j].min(dp[i - 1][j - v[i]] + w[i]);
            }
        }
    }

    for v in (0..=max_total_v).rev() {
        if dp[n - 1][v] <= m {
            println!("{}", v);
            return;
        }
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
