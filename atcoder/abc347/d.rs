#![allow(unused)]

use std::collections::HashMap;

fn main() {
    // dp[i, m, n] = is it possible to make:
    //     1. x[..=i] ^ y[..=i] = c[..=i]
    //     2. x[..=i].count_ones() == m
    //     3. y[..=i].count_ones() == n
    let inp = readv::<u64>();
    let a = inp[0] as usize;
    let b = inp[1] as usize;
    let c = format!("{:062b}", inp[2]).chars().collect::<Vec<_>>();
    let c = mapv(&c, |&cc| cc as usize - '0' as usize);
    let n = c.len();

    let mut dp = vec![vec![vec![false; b + 1]; a + 1]; n];

    // X: 0, Y: 0
    dp[0][0][0] = c[0] == 0;
    // X: 0, Y: 1
    if b >= 1 {
        dp[0][0][1] = c[0] == 1;
    }
    // X: 1, Y: 0
    if a >= 1 {
        dp[0][1][0] = c[0] == 1;
    }
    // X: 1, Y: 1
    if a >= 1 && b >= 1 {
        dp[0][1][1] = c[0] == 0;
    }

    let mut par = HashMap::new();

    for i in 1..n {
        for m in 0..=a {
            for n in 0..=b {
                for x in [0, 1] {
                    for y in [0, 1] {
                        if c[i] == x ^ y && m >= x && n >= y {
                            if dp[i - 1][m - x][n - y] {
                                dp[i][m][n] = true;
                                par.insert((i, m, n), (x, y));
                            }
                        }
                    }
                }
            }
        }
    }

    if !dp[n - 1][a][b] {
        println!("-1");
        return;
    }

    let mut x: u64 = 0;
    let mut y: u64 = 0;
    let mut curr_i = n - 1;
    let mut curr_m = a;
    let mut curr_n = b;
    while let Some(&(xx, yy)) = par.get(&(curr_i, curr_m, curr_n)) {
        // x[curr_i] = xx;
        // y[curr_i] = yy;
        x += (1u64 << (n - 1 - curr_i)) * xx as u64;
        y += (1u64 << (n - 1 - curr_i)) * yy as u64;
        if curr_i >= 1 && curr_m >= xx && curr_n >= yy {
            curr_i -= 1;
            curr_m -= xx;
            curr_n -= yy;
        } else {
            break;
        }
    }

    println!("{} {}", x, y);
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
