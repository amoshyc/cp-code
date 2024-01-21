#![allow(unused)]

const MOD: u64 = 998244353;

fn main() {
    let inp = readv::<u64>();
    let (n, m) = (inp[0] as usize, inp[1]);

    // dp[i, 0/1] = number of ways to fill A[..=i] while A[i] isn't/is same as A[0]
    // dp[0][1] = m since A[0] can be any of m color.
    // Consider the color of A[i],
    // 1. if A[i] is not the same color as A[0], then A[i - 1] can be the same or not.
    //    1a. if A[i - 1] is not the same A[0], then we can fill A[i] with m - 2 color.
    //    1b. if A[i - 1] is the same as A[0], then we can fill A[i] with m - 1 color.
    // 2. if A[i] is the same color as A[0], then A[i - 1] must not, therefore, dp[i][1] = dp[i - 1][0]
    let mut dp = vec![vec![0, 0]; n];
    dp[0][1] = m;
    for i in 1..n {
        dp[i][0] = dp[i - 1][0] * (m - 2) + dp[i - 1][1] * (m - 1);
        dp[i][1] = dp[i - 1][0];

        dp[i][0] %= MOD;
        dp[i][1] %= MOD;
    }

    println!("{}", dp[n - 1][0]);
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
