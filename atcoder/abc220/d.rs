#![allow(unused)]

const MOD: u64 = 998244353;

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    // dp[i, j] = number of ways after i operations the left most item is j
    let mut dp = vec![vec![0; 10]; n];
    dp[0][arr[0]] = 1;
    for i in 0..(n - 1) {
        for j in 0..10 {
            // F
            let k = (j + arr[i + 1]) % 10;
            dp[i + 1][k] += dp[i][j];
            dp[i + 1][k] %= MOD;
            // G
            let k = (j * arr[i + 1]) % 10;
            dp[i + 1][k] += dp[i][j];
            dp[i + 1][k] %= MOD;
        }
    }

    println!("{}", join(&dp[n - 1], "\n"));
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
