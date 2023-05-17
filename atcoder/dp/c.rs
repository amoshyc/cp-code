#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(readv::<u32>());
    }

    // dp[i, j] = max total points from day 0 to day i, while choosing activity j on day i.
    // dp[i, j] = max(dp[i - 1, k] for all k != j) + arr[i][j]
    let mut dp = vec![vec![0, 0, 0]; n];
    dp[0][0] = arr[0][0];
    dp[0][1] = arr[0][1];
    dp[0][2] = arr[0][2];
    for i in 1..n {
        for j in 0..3 {
            for k in 0..3 {
                if j != k {
                    dp[i][j] = dp[i][j].max(dp[i - 1][k] + arr[i][j]);
                }
            }
        }
    }
    println!("{}", dp[n - 1].iter().max().unwrap());
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
