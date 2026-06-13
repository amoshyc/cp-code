#![allow(unused)]

use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    }

    let n = s.len();
    let s = s
        .iter()
        .map(|&c| c as usize - 'a' as usize)
        .collect::<Vec<_>>();

    // dp[i] = number of valid subseq ends at i.
    // dp[i] = sum(dp[j] for j in 0..i if s[i] != s[j]) + 1

    let mut dp = vec![0; n];
    dp[0] = 1;

    let mut pref = vec![0; 26];
    pref[s[0]] = 1;

    for i in 1..n {
        dp[i] = 1;
        for c in 0..26 {
            if c != s[i] {
                dp[i] += pref[c];
                dp[i] %= 998244353;
            }
        }

        pref[s[i]] += dp[i];
        pref[s[i]] %= 998244353;
    }

    let mut ans = 0;
    for i in 0..n {
        ans += dp[i];
        ans %= 998244353;
    }

    println!("{}", ans);
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
