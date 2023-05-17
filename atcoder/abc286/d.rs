#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, x) = (inp[0], inp[1]);
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        a.push(inp[0]);
        b.push(inp[1]);
    }
    
    let mut dp = vec![vec![false; x + 1]; n];
    for c in 0..=b[0] {
        if c * a[0] <= x {
            dp[0][c * a[0]] = true;
        }
    }

    for i in 1..n {
        for j in 0..=x {
            for c in 0..=b[i] {
                if j >= c * a[i] && dp[i - 1][j - c * a[i]] {
                    dp[i][j] = true;
                }
            }
        }
    }

    if dp[n - 1][x] {
        println!("Yes");
    } else {
        println!("No");
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
