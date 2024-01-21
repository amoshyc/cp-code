#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut arr = vec![];
    for _ in 0..n {
        arr.push(reads());
    }

    let mut dp = vec![vec![0; m]; n];
    // dp[r, c] = max(dp[r + 1][c], dp[r][c + 1]) + 1
    for r in (0..n).rev() {
        for c in (0..m).rev() {
            if arr[r][c] == '#' {
                dp[r][c] = 0;
            } else {
                let mut next = 0;
                if r + 1 < n && arr[r + 1][c] == '.' {
                    next = next.max(dp[r + 1][c]);
                }
                if c + 1 < m && arr[r][c + 1] == '.' {
                    next = next.max(dp[r][c + 1]);
                }
                dp[r][c] = next + 1;
            }
        }
    }

    println!("{}", dp[0][0]);
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
