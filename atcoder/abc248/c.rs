#![allow(unused)]

fn main() {
    // dp[i, j] = number of A[0..=i] that has sum j
    // dp[i, j] = dp[i - 1, j - x] for x in 1..=m

    let inp = readv::<usize>();
    let (n, m, k) = (inp[0], inp[1], inp[2]);
    let mm = 998_244_353;
    let mut dp = vec![vec![0 as u64; k + 1]; n];
    
    for x in 1..=m {
        dp[0][x] = 1;
    }

    for i in 1..n {
        for j in 0..=k {
            for x in 1..=m {
                if j >= x {
                    dp[i][j] += dp[i - 1][j - x];
                    dp[i][j] %= mm;
                }
            }
        }
    }

    let mut ans = 0;
    for j in 0..=k {
        ans += dp[n - 1][j];
        ans %= mm;
    }

    println!("{}", ans);
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