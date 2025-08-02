#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (n, h, m) = (inp[0], inp[1], inp[2]);
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        a.push(inp[0]);
        b.push(inp[1]);
    }

    // dp[i, j] = maximum remaining magic when arriving at i with j remaining health
    // dp[i, j] = max(dp[i - 1, j + A[i]], dp[i - 1, j] - B[i])

    let mut dp = vec![vec![0; h + 1]; n];
    let mut cnt = 0;
    if h >= a[0] {
        dp[0][h - a[0]] = m;
        cnt = 1;
    }
    if m >= b[0] {
        dp[0][h] = m - b[0];
        cnt = 1;
    }

    if cnt == 0 {
        println!("0");
        return;
    }

    for i in 1..n {
        let mut any = false;
        for j in 0..=h {
            if j + a[i] <= h && dp[i - 1][j + a[i]] > 0 {
                dp[i][j] = dp[i][j].max(dp[i - 1][j + a[i]]);
                any = true;
            }
            if dp[i - 1][j] >= b[i] {
                dp[i][j] = dp[i][j].max(dp[i - 1][j] - b[i]);
                any = true;
            }
        }
        if any {
            cnt += 1;
        } else {
            break;
        }
    }

    println!("{}", cnt);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
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
