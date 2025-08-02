#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, w) = (inp[0], inp[1]);
    let mut arr = vec![vec![]; h];
    for r in 0..h {
        arr[r] = readv::<i64>();
    }
    let p = readv::<i64>();

    let inf = 10i64.pow(18);
    let mut dp = vec![vec![inf; w]; h];
    for r in (0..h).rev() {
        for c in (0..w).rev() {
            let atleast = (p[r + c] - arr[r][c]).max(0);
            if (r, c) == (h - 1, w - 1) {
                dp[h - 1][w - 1] = atleast;
                continue;
            }
            if c + 1 < w {
                let v = (dp[r][c + 1] - arr[r][c] + p[r + c]).max(0);
                dp[r][c] = dp[r][c].min(v);
            }
            if r + 1 < h {
                let v = (dp[r + 1][c] - arr[r][c] + p[r + c]).max(0);
                dp[r][c] = dp[r][c].min(v);
            }
            dp[r][c] = dp[r][c].max(atleast);
        }
    }
    println!("{}", dp[0][0]);
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    s.trim().parse().ok().unwrap()
}

fn readv<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_ascii_whitespace()
        .map(|t| t.parse().ok().unwrap())
        .collect()
}

fn reads() -> Vec<char> {
    read::<String>().chars().collect()
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
