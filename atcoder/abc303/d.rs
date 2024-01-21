#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (x, y, z) = (inp[0], inp[1], inp[2]);
    let s = reads();
    let n = s.len();

    // dp[i, 0/1] = min time to make s[..i] while cap is on/off

    let inf = 10i64.pow(18);
    let mut dp = vec![vec![inf, inf]; n + 1];

    // if s[0] == 'a' {
    //     dp[0][0] = x;
    //     dp[0][1] = x + z;
    // } else {
    //     dp[0][0] = std::cmp::min(y, z + x + z);
    //     dp[0][1] = std::cmp::min(y + z, z + x);
    // }
    dp[0][0] = 0;
    dp[0][1] = inf;

    for i in 0..n {
        if s[i] == 'a' {
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][0] + x);
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][1] + z + x);
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][1] + y + z);

            dp[i + 1][1] = dp[i + 1][1].min(dp[i][0] + x + z);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][0] + z + y);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][1] + z + x + z);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][1] + y);
        } else {
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][0] + y);
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][0] + z + x + z);
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][1] + z + x);
            dp[i + 1][0] = dp[i + 1][0].min(dp[i][1] + z + y);

            dp[i + 1][1] = dp[i + 1][1].min(dp[i][0] + y + z);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][0] + z + x);
            dp[i + 1][1] = dp[i + 1][1].min(dp[i][1] + x);
        }
    }

    println!("{}", dp[n][0].min(dp[n][1]));
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
