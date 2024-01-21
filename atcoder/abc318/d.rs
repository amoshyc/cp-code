#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let mut d = vec![vec![0; n]; n];
    for i in 0..(n - 1) {
        let inp = readv::<i64>();
        for (j, x) in inp.iter().enumerate() {
            d[i][i + j + 1] = *x;
        }
    }

    // dp[S] = maximum total weight when using only vertice set S
    // dp[S] = max(dp[S - (u, v)] + d[u][v] for (u, v) in S)
    let mut dp = vec![0; (1 << n)];
    dp[0] = 0;

    for s in 1..(1 << n) {
        for u in 0..n {
            if (s >> u) & 1 == 1 {
                for v in (u + 1)..n {
                    if (s >> v) & 1 == 1 {
                        dp[s] = dp[s].max(dp[s ^ (1 << u) ^ (1 << v)] + d[u][v]);
                    }
                }
            }
        }
    }

    println!("{}", dp[(1 << n) - 1]);
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
