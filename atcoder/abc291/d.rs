#![allow(unused)]

fn main() {
    // let dp[i, j] = number of ways of first i + 1 items and i-th is A[i]/B[i]

    let n = read::<usize>();
    let m: u64 = 998244353;
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let inp = readv::<u64>();
        a.push(inp[0]);
        b.push(inp[1]);
    }

    let mut dp = vec![vec![0, 0]; n];
    dp[0][0] = 1;
    dp[0][1] = 1;

    for i in 1..n {
        let item1 = if a[i] != a[i - 1] { dp[i - 1][0] } else { 0 };
        let item2 = if a[i] != b[i - 1] { dp[i - 1][1] } else { 0 };
        dp[i][0] = item1 + item2;
        dp[i][0] %= m;

        let item1 = if b[i] != a[i - 1] { dp[i - 1][0] } else { 0 };
        let item2 = if b[i] != b[i - 1] { dp[i - 1][1] } else { 0 };
        dp[i][1] = item1 + item2;
        dp[i][1] %= m;
    }

    let ans = (dp[n - 1][0] + dp[n - 1][1]) % m;
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
