#![allow(unused)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(i64, usize); n],
    }

    let a = ab.iter().map(|(a, b)| *a).collect::<Vec<i64>>();
    let b = ab.iter().map(|(a, b)| *b).collect::<Vec<usize>>();

    // dp[i, j, 0/1] = maximum profit of towns 0..=i with j expenses left while don't/do business in town i

    // dp[i, j, 0] = max(dp[i - 1, j, 0], dp[i - 1, j, 1])
    // dp[i, j, 1] = max(dp[p, j + b[i], 1] + a[i] | i - k <= p < i)
    //             = (can be improved using segtree)

    let mut dp = vec![vec![vec![0, 0]; m + 1]; n];

    dp[0][m][0] = 0;
    dp[0][m - b[0]][1] = a[0];

    for i in 1..n {
        for j in 0..=m {
            // dp[i, j, 0]
            dp[i][j][0] = dp[i - 1][j][0].max(dp[i - 1][j][1]);

            // dp[i, j, 1]
            for p in i.saturating_sub(k)..i {
                if j + b[i] <= m {
                    dp[i][j][1] = dp[i][j][1].max(dp[p][j + b[i]][1] + a[i]);
                }
            }
        }
    }

    let mut ans = 0;
    for j in 0..=m {
        ans = ans.max(dp[n - 1][j][0]).max(dp[n - 1][j][1]);
    }
    println!("{ans}");
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
