#![allow(unused)]

use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        tc: usize
    }

    let mut ans = vec![];
    for _ in 0..tc {
        input! {
            n: usize,
            m: usize,
            k: usize,
            s: Chars,
            es: [(Usize1, Usize1); m],
        }

        let mut dp = vec![vec![!0; n]; 2 * k + 1];
        let mut adj = vec![vec![]; n];
        for (u, v) in es {
            adj[u].push(v);
        }

        if dfs(2 * k, 0, &mut dp, &s, &adj) == 0 {
            ans.push("Alice");
        } else {
            ans.push("Bob");
        }
    }

    println!("{}", join(&ans, "\n"));
}

// There are r rounds left (r is even: alice's turn, r is odd: bob's turn)
// Return the winner (0: alice, 1: bob)
fn dfs(
    r: usize,
    u: usize,
    dp: &mut Vec<Vec<usize>>,
    s: &Vec<char>,
    adj: &Vec<Vec<usize>>,
) -> usize {
    if r == 0 {
        if s[u] == 'A' {
            return 0;
        } else {
            return 1;
        }
    }

    if dp[r][u] != !0 {
        return dp[r][u];
    }

    // if there is a way to achieve `who`'s winning state, go with it.
    let who = r % 2;
    for &v in &adj[u] {
        if dfs(r - 1, v, dp, s, adj) == who {
            dp[r][u] = who;
            return dp[r][u];
        }
    }
    dp[r][u] = 1 - who;
    dp[r][u]
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
