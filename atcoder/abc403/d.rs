#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let inp = readv::<usize>();
    let (n, d) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    let max = *arr.iter().max().unwrap();
    let mut cnt = vec![0; max + 1];
    for i in 0..n {
        cnt[arr[i]] += 1;
    }

    if d == 0 {
        // arr[i] != arr[j] for i != j
        // => all elements in arr is distinct
        // => remove duplicated ones
        let ans = (0..=max)
            .filter(|i| cnt[*i] > 0)
            .map(|i| cnt[i] - 1)
            .sum::<usize>();
        println!("{}", ans);
        return;
    }

    let mut ans = 0;
    for i in 0..=max {
        // i, i + d, i + 2d, i + 3d, ...
        let mut path = vec![];
        for x in (i..=max).step_by(d) {
            if cnt[x] > 0 {
                path.push(cnt[x]);
                cnt[x] = 0;
            } else {
                break;
            }
        }

        if path.len() == 0 {
            continue;
        }

        // Find the min total sum if for every adjacent elements we must choose at least one
        // dp[i, 0/1] = min total sum of path[..=i] if path[i] is selected (1) or not (0)
        let m = path.len();
        let mut dp = vec![vec![n + 1, n + 1]; m];
        dp[0][0] = 0;
        dp[0][1] = path[0];
        for i in 1..m {
            // path[i] is not selected -> path[i - 1] must be selected
            dp[i][0] = dp[i - 1][1];
            // path[i] is selected -> path[i - 1] may or may not be selected
            dp[i][1] = dp[i - 1][0].min(dp[i - 1][1]) + path[i];
        }
        ans += dp[m - 1][0].min(dp[m - 1][1]);
    }

    println!("{}", ans);
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
