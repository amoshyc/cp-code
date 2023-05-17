#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i32>();

    // dp[i] = min total cost from 0 to i
    // dp[i] = min(
    //     dp[i - 1] + abs(A[i] - A[i - 1]),
    //     dp[i - 2] + abs(A[i] - A[i - 2]),
    // )
    let mut dp = vec![1_000_000_000; n];
    dp[0] = 0;
    for i in 1..n {
        if i >= 1 {
            dp[i] = dp[i].min(dp[i - 1] + (arr[i] - arr[i - 1]).abs());
        }
        if i >= 2 {
            dp[i] = dp[i].min(dp[i - 2] + (arr[i] - arr[i - 2]).abs());
        }
    }
    println!("{}", dp[n - 1]);
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
