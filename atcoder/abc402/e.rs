#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, x) = (inp[0], inp[1]);
    let mut scp = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        let (s, c, p) = (inp[0] as f64, inp[1], inp[2] as f64 / 100.0);
        scp.push((s, c, p));
    }

    let mut dp = vec![vec![-1.0; x + 1]; 1 << n];
    let ans = dfs(0, x, &scp, &mut dp);
    println!("{:.7}", ans);
}

fn dfs(mask: usize, rem: usize, scp: &Vec<(f64, usize, f64)>, dp: &mut Vec<Vec<f64>>) -> f64 {
    if dp[mask][rem] >= 0.0 {
        return dp[mask][rem];
    }

    let avail: Vec<usize> = (0..scp.len()).filter(|i| (mask >> i) & 1 == 0).collect();

    if avail.len() == 0 {
        return 0.0;
    }

    let mut max: f64 = 0.0;
    for i in avail {
        let (s, c, p) = scp[i];
        if c <= rem {
            let mut sum = 0.0;
            sum += p * (s + dfs(mask | (1 << i), rem - c, scp, dp));
            sum += (1.0 - p) * dfs(mask, rem - c, scp, dp);
            max = max.max(sum);
        }
    }

    dp[mask][rem] = max;

    max
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
