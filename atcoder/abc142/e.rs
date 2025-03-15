#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);

    let mut costs = vec![0; m];
    let mut masks = vec![0; m];
    for i in 0..m {
        let inp = readv::<usize>();
        let (a, b) = (inp[0], inp[1]);
        let c = readv::<usize>();
        costs[i] = a as i64;
        for j in 0..b {
            masks[i] |= 1 << (c[j] - 1);
        }
    }

    // dp[i][S] = minimum cost to unlock set S using keys 0..=i
    let inf = 10i64.pow(18);
    let mut dp = vec![inf; (1 << n)];
    dp[0] = 0;

    for i in 0..m {
        for s in 0..(1 << n) {
            if dp[s] != inf {
                dp[s | masks[i]] = dp[s | masks[i]].min(dp[s] + costs[i]);
            }
        }
    }

    if dp[(1 << n) - 1] == inf {
        println!("-1");
    } else {
        println!("{}", dp[(1 << n) - 1]);
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
