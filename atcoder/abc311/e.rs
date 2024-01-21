#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w, n) = (inp[0], inp[1], inp[2]);
    let mut arr = vec![vec![0; w]; h];
    for _ in 0..n {
        let inp = readv::<usize>();
        let (r, c) = (inp[0] - 1, inp[1] - 1);
        arr[r][c] = 1;
    }

    // dp[r, c] = max length of square with bottom right at (r, c)
    // dp[r, c] = min(dp[r, c - 1], dp[r - 1, c], dp[r - 1, c - 1]) + 1
    let mut dp = vec![vec![0 as i64; w]; h];
    for c in 0..w {
        if arr[0][c] == 0 {
            dp[0][c] = 1;
        }
    }
    for r in 0..h {
        if arr[r][0] == 0 {
            dp[r][0] = 1;
        }
    }
    for r in 1..h {
        for c in 1..w {
            if arr[r][c] == 0 {
                dp[r][c] = dp[r - 1][c].min(dp[r][c - 1]).min(dp[r - 1][c - 1]) + 1;
            }
        }
    }

    let mut ans = 0;
    for r in 0..h {
        for c in 0..w {
            ans += dp[r][c];
        }
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

fn mapv<T, S, F: Fn(&T) -> S>(arr: &Vec<T>, f: F) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
