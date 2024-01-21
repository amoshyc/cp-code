#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i64>();

    // average
    // 1 1 1 0 0 0
    let mut lb = 0.0;
    let mut ub = 1e15;
    let check = |m: f64| -> bool {
        // avg >= m
        // sum(arr[i] for i in S) / sum(1 for i in S) >= m
        // sum(arr[i] - m for i in S) >= 0
        // dp[i] = max subseq sum when choosing from 0..=i while i is chosen
        // dp[i] = max(dp[i - 1], dp[i - 2]) + (arr[i] - m)
        // ans = max(dp[n - 1], dp[n - 2]) >= 0.0
        let mut dp = vec![0.0; n];
        dp[0] = arr[0] as f64 - m;
        dp[1] = dp[0].max(0.0) + arr[1] as f64 - m;
        for i in 2..n {
            dp[i] = dp[i - 1].max(dp[i - 2]) + arr[i] as f64 - m;
        }
        dp[n - 1].max(dp[n - 2]) >= 0.0
    };
    for _ in 0..100 {
        let m = (lb + ub) / 2.0;
        if check(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    println!("{:.12}", lb);

    // median
    // 1 1 1 0 0 0
    let mut lb = 0;
    let mut ub = 1_000_000_000 + 1;
    let check = |m: i64| -> bool {
        // median >= m
        // (number of elem >= m) > (number of elem < m)
        // (number of elem >= m) - (number of elem < m) > 0
        // dp[i] = number of more >= m than < m when choosing from 0..=i while i is chosen
        // dp[i] = max(dp[i - 1], dp[i - 2]) + 1 if arr[i] >= m else -1
        let mut dp = vec![0; n];
        dp[0] = if arr[0] >= m { 1 } else { -1 };
        dp[1] = dp[0].max(0) + if arr[1] >= m { 1 } else { -1 };
        for i in 2..n {
            dp[i] = dp[i - 1].max(dp[i - 2]) + if arr[i] >= m { 1 } else { -1 };
        }
        dp[n - 1].max(dp[n - 2]) > 0
    };
    while ub - lb > 1 {
        let m = (lb + ub) / 2;
        if check(m) {
            lb = m;
        } else {
            ub = m;
        }
    }
    println!("{}", lb);
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
