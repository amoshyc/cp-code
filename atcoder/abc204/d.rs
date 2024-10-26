#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let t = readv::<usize>();
    let s = t.iter().sum::<usize>();

    // dp[i, j] = are there a subset from T[0..=i] that has sum j.
    // dp[i, j] = dp[i - 1][j] or dp[i - 1][j - t[i]]
    let mut dp = vec![vec![false; s / 2 + 1]; n];
    dp[0][0] = true;
    if t[0] <= s / 2 {
        dp[0][t[0]] = true;
    }
    for i in 1..n {
        for j in 0..=(s / 2) {
            dp[i][j] = dp[i - 1][j];
            if j >= t[i] {
                dp[i][j] |= dp[i - 1][j - t[i]];
            }
        }
    }

    for j in (0..=(s / 2)).rev() {
        if dp[n - 1][j] {
            println!("{}", s - j);
            break;
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
    read::<String>().chars().collect()
}

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
