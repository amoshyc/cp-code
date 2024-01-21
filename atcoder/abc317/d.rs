#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut x = vec![];
    let mut y = vec![];
    let mut z = vec![];
    for _ in 0..n {
        let inp = readv::<i64>();
        x.push(inp[0]);
        y.push(inp[1]);
        z.push(inp[2] as usize);
    }

    // dp[i, j] = number of switch from A[0..=i] to make Takahashi win while total z is j
    // dp[i, j] = min(dp[i - 1, j], dp[i - 1, j - z[i]] + w[i])

    let inf = i64::MAX / 2;
    let z_sum = z.iter().sum::<usize>();
    let mut dp = vec![inf; z_sum + 1];
    dp[0] = 0;
    for i in 0..n {
        let w = if x[i] > y[i] {
            0
        } else {
            (x[i] + y[i] + 1) / 2 - x[i]
        };
        for j in (z[i]..=z_sum).rev() {
            dp[j] = dp[j].min(dp[j - z[i]] + w);
        }
    }

    let p = (z_sum + 1) / 2;
    let ans = dp[p..].iter().min().unwrap();
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
