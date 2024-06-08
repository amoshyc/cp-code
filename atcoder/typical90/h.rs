#![allow(unused)]

// [Problem]
// You are given a string S of length N consisting of lowercase English letters.
// Let si​ denote the i-th character of S.
// There are 2^N ways to extract zero or more characters from S, but how many of them satisfy the following condition?
// Since the answer could be very large, output the remainder when divided by 10^9+7.
//     The concatenated string formed by extracting characters in order is "atcoder".
// However, the "ways to extract characters" are distinguished if there is at least one character si​ (1≤i≤N) that is extracted from only one side.

// [Solution]
// Let T = "atcoder". Let dp[i, j] = number of way to extract T[..j] from S[..i].
// The answer is dp[N, 8].
// The base case is dp[i, 0] = 1 since there is a way to select empty string from any string.
// By considering the char of S[i - 1] and T[j - 1], we get:
// if S[i - 1] != T[j - 1], then selecting T[..j] from S[..i] is same as selecting T[..j] from S[..i - 1].
// if S[i - 1] == T[j - 1], then selecting T[..j] from S[..i] can have 2 cases.
// Case 1: Selecting T[..j - 1] from S[..i - 1], and pick S[i - 1].
// Case 2: Selecting T[..j] from S[..i - 1], and pick S[i - 1]. This deals with the repeat of S[i - 1].

fn main() {
    let n = read::<usize>();
    let s = reads();
    let t = "atcoder".chars().collect::<Vec<char>>();
    let m = 10i64.pow(9) + 7;

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 0..=s.len() {
        dp[i][0] = 1;
    }

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1]) % m;
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    println!("{}", dp[s.len()][t.len()]);
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
    read::<String>().chars().collect::<_>()
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
