#![allow(unused)]

const MOD: u64 = 1_000_000_007;

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    // dp[r, c] = number of ways to go to (r, c) from (0, 0)
    let mut dp = vec![vec![0; m]; n];
    for r in 0..n {
        for c in 0..m {
            if arr[r][c] == '#' {
                continue;
            }
            if r == 0 && c == 0 {
                dp[r][c] = 1;
                continue;
            }
            if r >= 1 && arr[r - 1][c] == '.' {
                dp[r][c] = dp[r][c] + dp[r - 1][c];
                dp[r][c] %= MOD;
            }
            if c >= 1 && arr[r][c - 1] == '.' {
                dp[r][c] = dp[r][c] + dp[r][c - 1];
                dp[r][c] %= MOD;
            }
        }
    }

    println!("{}", dp[n - 1][m - 1]);
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
