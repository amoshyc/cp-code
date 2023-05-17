#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, k) = (inp[0], inp[1]);
    let arr = readv::<usize>();

    // dp[i, 0] = whether Taro wins when there are i stones and it's Taro's turn.
    // dp[i, 1] = whether Taro wins when there are i stones and it's Jiro's turn.

    // dp[i, 0] = True if any(dp[i - a, 1] is False for a in A) else False
    // dp[i, 1] = True if all(dp[i - a, 0] is True for a in A) else False

    let mut dp = vec![vec![false, false]; k + 1];
    dp[0][0] = false;
    dp[0][1] = true;
    for i in 1..=k {
        dp[i][0] = arr.iter().filter(|&&a| i >= a).any(|&a| dp[i - a][1]);
        dp[i][1] = arr.iter().filter(|&&a| i >= a).all(|&a| dp[i - a][0]);
    }
    if dp[k][0] {
        println!("First");
    } else {
        println!("Second");
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

fn join<T: ToString>(arr: &[T], sep: &str) -> String {
    arr.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
