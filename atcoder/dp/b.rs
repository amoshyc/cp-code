#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<i64>();

    // dp[i] = min total cost from 0 to i
    // dp[i] = min{ dp[i - j] + abs(A[i] - A[i - j]) for j in 1..=k }
    let mut dp = vec![1_000_000_000_000; n];
    dp[0] = 0;
    for i in 1..n {
        for j in 1..=k {
            if i >= j {
                dp[i] = dp[i].min(dp[i - j] + (arr[i] - arr[i - j]).abs());
            }
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
