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
    let mut dp = vec![vec![false; 10000 + 1]; n];
    dp[0][a[0]] = true;
    dp[0][b[0]] = true;
    for i in 1..n {
        for j in 0..=x {
            if j >= a[i] && dp[i - 1][j - a[i]] {
                dp[i][j] = true;
            }
            if j >= b[i] && dp[i - 1][j - b[i]] {
                dp[i][j] = true;
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
