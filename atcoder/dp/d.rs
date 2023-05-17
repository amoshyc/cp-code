#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut w = vec![];
    let mut v = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        w.push(inp[0]);
        v.push(inp[1] as u64);
    }

    // dp[i][j] = max total value using items 0..=i while total weight = j
    // dp[i][j] = max(
    //     dp[i - 1][j - w[i]] + v[i]  # pick item i
    //     dp[i - 1][j]                # don't pick item i
    // )
    let mut dp = vec![vec![0; m + 1]; n];
    dp[0][w[0]] = v[0];
    for i in 1..n {
        for j in 0..=m {
            dp[i][j] = dp[i - 1][j];
            if j >= w[i] {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - w[i]] + v[i]);
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
