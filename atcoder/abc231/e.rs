#![allow(unused)]

fn main() {
    // c is carry, r is change, p is pay, v is price.
    //     c3  c2  c1   0
    //     v3  v2  v1  v0
    //  +) r3  r2  r1  r0
    // ------------------
    //     p3  p2  p1  p0

    // c[i + 1] = floor_div(c[i] + v[i] + r[i], m[i])
    // p[i] = (c[i] + v[i] + r[i]) mod m[i]
    // m[i] = A[i + 1] / A[i]

    // dp[i, c] = minimum number of coins needed to fill r[0..i] and p[0..i] and c[i] = c
    // dp[0, 0] = 0
    // answer = dp[N, 0]

    // for i in 0..n:
    //     for c[i] in 0..2:
    //         for r[i] in 0..m[i]:
    //             dp[i + 1, c[i + 1]].chmin(dp[i, c] + r[i] + p[i])

    // There are limited (c[i], r[i]) pairs.
    // c[i + 1] = floor_div(c[i] + v[i] + r[i], m[i])
    //          = 0 if c[i] + v[i] + r[i] < m[i] else 1
    //          = 0 if r[i] < m[i] - v[i] - c[i] else 1
    // That is,
    // for all r[i] < m[i] - v[i] - c[i], they all maps to dp[i + 1, 0]
    // for all r[i] >= m[i] - v[i] - c[i], they all maps to dp[i + 1, 1]

    // For r[i] < m[i] - v[i] - c[i], we have c[i] + v[i] + r[i] < m[i], so
    //     dp[i + 1, 0].chmin(dp[i, c] + r[i] + p[i])
    //     dp[i + 1, 0].chmin(dp[i, c] + r[i] + ((c[i] + v[i] + r[i]) mod m[i]))
    //     dp[i + 1, 0].chmin(dp[i, c] + r[i] + c[i] + v[i] + r[i])
    //     dp[i + 1, 0].chmin(dp[i, c] + c[i] + v[i] + 2 r[i])
    // Minimum occurs when r[i] = 0 (r[i] >= 0)

    // For r[i] >= m[i] - v[i] - c[i], we have c[i] + v[i] + r[i] >= m[i], so
    //     dp[i + 1, 1].chmin(dp[i, c] + r[i] + p[i])
    //     dp[i + 1, 1].chmin(dp[i, c] + r[i] + ((c[i] + v[i] + r[i]) mod m[i]))
    //     dp[i + 1, 1].chmin(dp[i, c] + r[i] + c[i] + v[i] + r[i] - m[i])
    //     dp[i + 1, 1].chmin(dp[i, c] + c[i] + v[i] + 2 r[i] - m[i])
    // Minimum occurs when r[i] = m[i] - v[i] - c[i] (r[i] >= 0)

    let inf = i64::MAX / 4;

    let inp = readv::<i64>();
    let arr = readv::<i64>();
    let n = inp[0] as usize;

    // Represent X in A format
    let mut x = inp[1];
    let mut v = vec![];
    for i in 0..n {
        v.push(x / arr[n - 1 - i]);
        x %= arr[n - 1 - i];
    }
    v.reverse();
    // pad v
    while v.len() < n {
        v.push(0);
    }

    let mut m = vec![inf; n];
    for i in 0..(n - 1) {
        m[i] = arr[i + 1] / arr[i];
    }
    
    let mut dp = vec![vec![inf; 2]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for c in 0..2 {
            if dp[i][c] == inf {
                continue;
            }

            let c_i = c as i64;

            let r_i = 0;
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][c] + c_i + v[i] + 2 * r_i);

            let r_i = (m[i] - c_i - v[i]).max(0);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][c] + c_i + v[i] + 2 * r_i - m[i]);
        }
    }

    println!("{}", dp[n][0]);
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
