#![allow(unused)]

use std::collections::VecDeque;

fn main() {
    let n = read::<usize>();
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let inf = 1_000_000;

    // dp[u][v] = minimum distance from u to v using palindrome path
    let mut dp = vec![vec![inf; n]; n];
    let mut que = VecDeque::new();

    for a in 0..n {
        dp[a][a] = 0;
        que.push_back((a, a));
    }

    for a in 0..n {
        for b in 0..n {
            if a != b && arr[a][b] != '-' {
                dp[a][b] = 1;
                que.push_back((a, b));
            }
        }
    }

    while let Some((s, t)) = que.pop_front() {
        for u in 0..n {
            for v in 0..n {
                // u -> s ~> t -> v
                if arr[u][s] == arr[t][v] && arr[u][s] != '-' && arr[t][v] != '-' {
                    if dp[s][t] + 2 < dp[u][v] {
                        dp[u][v] = dp[u][v].min(dp[s][t] + 2);
                        que.push_back((u, v));
                    }
                }
            }
        }
    }

    for r in 0..n {
        let row = mapv(&dp[r], |&x| if x == inf { -1 } else { x });
        println!("{}", join(&row, " "));
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
