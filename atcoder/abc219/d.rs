#![allow(unused)]

// 2
// 59 3
// 163 2
// 105 1

fn main() {
    let n: usize = read();
    let inp = readv::<usize>();
    let (x, y) = (inp[0], inp[1]);
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        a.push(inp[0]);
        b.push(inp[1]);
    }

    let inf = 1_000;
    let mut dp = vec![vec![vec![inf; y + 1]; x + 1]; n];
    // dp[i, j, k] = min boxes to get >= j & >= k using A[0..=i]

    dp[0][0][0] = 0;
    for j in 0..=a[0].min(x) {
        for k in 0..=b[0].min(y) {
            dp[0][j][k] = dp[0][j][k].min(1);
        }
    }
    for i in 1..n {
        for j in 0..=x {
            for k in 0..=y {
                dp[i][j][k] = dp[i - 1][j][k];
                let prev_j = if j >= a[i] { j - a[i] } else { 0 };
                let prev_k = if k >= b[i] { k - b[i] } else { 0 };
                dp[i][j][k] = dp[i][j][k].min(dp[i - 1][prev_j][prev_k] + 1);
            }
        }
    }

    if dp[n - 1][x][y] == inf {
        println!("-1");
    } else {
        println!("{}", dp[n - 1][x][y]);
    }
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
