#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let p = readv::<f64>();

    // dp[i, j] = Pr(head > tail) when tossing coins 0..=i and get j heads
    // dp[i, j] = p[i] * dp[i - 1, j - 1] + (1 - p[i]) * dp[i - 1, j]
    let mut dp = vec![vec![0 as f64; n + 1]; n];
    dp[0][0] = 1.0 - p[0];
    dp[0][1] = p[0];
    for i in 1..n {
        for j in 0..=n {
            dp[i][j] = (1.0 - p[i]) * dp[i - 1][j];
            if j >= 1 {
                dp[i][j] += p[i] * dp[i - 1][j - 1];
            }
        }
    }

    let mut ans = 0.0;
    for j in 1..=n {
        if j > (n - j) {
            ans += dp[n - 1][j];
        }
    }

    println!("{:.12}", ans);
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
