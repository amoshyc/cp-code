#![allow(unused)]

const MOD: u64 = 1_000_000_007;

fn main() {
    // dp[i, j] = number of subsequence = t[..=j] from s[..=i]
    // dp[i, j] = dp[i - 1, j] + (dp[i - 1, j - 1] if t[j] == s[i] else 0)
    let s = reads();
    let t = "chokudai".chars().collect::<Vec<char>>();

    let mut dp = vec![0; 8];
    for i in 0..s.len() {
        let mut new = vec![0; 8];
        for j in 0..8 {
            new[j] = dp[j];
            if s[i] == t[j] {
                if j == 0 {
                    new[j] += 1;
                } else {
                    new[j] += dp[j - 1];
                }
            }
            new[j] %= MOD;
        }
        dp = new;
    }

    println!("{}", dp[7]);
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
