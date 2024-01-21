#![allow(unused)]

fn main() {
    // dp[i, j] = maximum rating if j elements are chosen from p[0..=i]
    // dp[0, 1] = P[0]
    // dp[i, j] = max(A, B)
    // (P[i] is used) A = ((dp[i - 1, j - 1] + term[j - 1]) * den[j - 1] * 0.9 + p[i]) / den[j] - term[j]
    // (P[i] not used) B = dp[i - 1, j]
    let n = read::<usize>();
    let p = readv::<f64>();

    let inf = 1e18;

    let mut den = vec![inf; n + 1];
    den[1] = 1.0;
    for k in 2..=n {
        den[k] = den[k - 1] * 0.9 + 1.0;
    }

    let mut term = vec![0.0; n + 1];
    for k in 1..=n {
        term[k] = 1200.0 / (k as f64).sqrt();
    }

    let mut dp = vec![vec![-inf; n + 1]; n];
    dp[0][0] = -inf;
    dp[0][1] = p[0] - term[1];
    for i in 1..n {
        dp[i][0] = -inf;
        dp[i][1] = dp[i - 1][1].max(p[i] - term[1]);
        for j in 2..=(i + 1) {
            let a = ((dp[i - 1][j - 1] + term[j - 1]) * den[j - 1] * 0.9 + p[i]) / den[j] - term[j];
            let b = dp[i - 1][j];
            dp[i][j] = a.max(b);
        }
    }

    let mut ans = -inf;
    for j in 1..=n {
        ans = ans.max(dp[n - 1][j]);
    }
    println!("{:.7}", ans);
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
