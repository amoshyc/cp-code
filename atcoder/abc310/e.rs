#![allow(unused)]

fn main() {
    // dp[i, 0/1] = number of pairs ending at i, and have 0/1 at the end
    let n = read::<usize>();
    let s = reads();
    let s = mapv(&s, |&c| if c == '0' { 0 } else { 1 });

    let mut dp = vec![vec![0i64, 0]; n];
    dp[0][s[0]] = 1;
    for i in 1..n {
        if s[i] == 1 {
            dp[i][0] = dp[i - 1][1];
            dp[i][1] = dp[i - 1][0];
        } else {
            dp[i][1] = dp[i - 1][0] + dp[i - 1][1];
            dp[i][0] = 0;
        }
        dp[i][s[i]] += 1;
    }

    let ans: i64 = (0..n).map(|i| dp[i][1]).sum();
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

fn mapv<T, F>(arr: &Vec<T>, f: fn(&T) -> F) -> Vec<F> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
