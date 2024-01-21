#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, k) = (inp[0] as usize, inp[1]);
    let arr = readv::<i64>();

    let mut dp = vec![vec![0; n]; 40];
    for u in 0..n {
        dp[0][u] = arr[u];
    }
    for i in 1..40 {
        for u in 0..n {
            let v = ((u as i64 + dp[i - 1][u]) % (n as i64)) as usize;
            dp[i][u] = dp[i - 1][u] + dp[i - 1][v];
        }
    }

    let mut s = 0;
    for i in 0..40 {
        if (k >> i) & 1 == 1 {
            s += dp[i][(s % (n as i64)) as usize];
        }
    }

    println!("{}", s);
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
