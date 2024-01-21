#![allow(unused)]

const MOD: i64 = 998244353;

fn main() {
    let n = read::<usize>();

    // dp[i, j] = number of i digit number while leftmost digit is j
    let mut dp = vec![vec![0; 10]; n + 1];
    for i in 1..10 {
        dp[1][i] = 1;
    }
    for i in 1..n {
        for j in 1..10 {
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= MOD;
            if j + 1 < 10 {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= MOD;
            }
            if j >= 1 {
                dp[i + 1][j - 1] += dp[i][j];
                dp[i + 1][j - 1] %= MOD;
            }
        }
    }

    let mut ans = 0;
    for j in 1..10 {
        ans += dp[n][j];
        ans %= MOD;
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
