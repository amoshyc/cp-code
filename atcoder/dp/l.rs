#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i64>();

    // f(i, j, 0) = the resulting value X - Y when sequence is a[i..=j] and it's Taro's turn.
    // f(i, j, 1) = the resulting value X - Y when sequence is a[i..=j] and it's Jiro's turn.

    // f(i, i, 0) = a[i]
    // f(i, i, 1) = -a[i]

    // f(i, j, 0) = max(f(i + 1, j, 1) + arr[i], f(i, j - 1, 1) + arr[j])
    // f(i, j, 1) = min(f(i + 1, j, 0) - arr[i], f(i, j - 1, 1) - arr[j])

    let mut dp = vec![vec![vec![0, 0]; n]; n];
    for i in 0..n {
        dp[i][i][0] = arr[i];
        dp[i][i][1] = -arr[i];
    }

    for l in 2..=n {
        for i in (0..).take_while(|&i| i + l <= n) {
            let j = i + l - 1;
            dp[i][j][0] = std::cmp::max(dp[i + 1][j][1] + arr[i], dp[i][j - 1][1] + arr[j]);
            dp[i][j][1] = std::cmp::min(dp[i + 1][j][0] - arr[i], dp[i][j - 1][0] - arr[j]);
        }
    }

    println!("{}", dp[0][n - 1][0]);
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
