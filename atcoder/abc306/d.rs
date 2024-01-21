#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut kind = vec![];
    let mut val = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        kind.push(inp[0]);
        val.push(inp[1]);
    }

    let inf = 10i64.pow(15);
    let mut dp = vec![vec![-inf, -inf]; n];

    // don't eat
    dp[0][0] = 0;
    // eat
    if kind[0] == 0 {
        dp[0][0] = dp[0][0].max(val[0]);
    } else {
        dp[0][1] = val[0];
    }

    for i in 1..n {
        // don't eat i
        dp[i][0] = dp[i - 1][0];
        dp[i][1] = dp[i - 1][1];

        // eat i
        if kind[i] == 0 {
            // antidotal
            dp[i][0] = dp[i][0].max(dp[i - 1][0].max(dp[i - 1][1]) + val[i]);
        } else {
            // poisonous
            dp[i][1] = dp[i][1].max(dp[i - 1][0] + val[i]);
        }
    }

    println!("{}", dp[n - 1][0].max(dp[n - 1][1]));
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
