#![allow(unused)]

fn main() {
    let inp = readv::<i64>();
    let (n, k) = (inp[0] as usize, inp[1]);
    let nxt = readv::<usize>();
    let nxt = mapv(&nxt, |x| *x - 1);
    let arr = readv::<usize>();

    let mut m = 1;
    while (1i64 << m) < k {
        m += 1;
    }

    let mut dp = vec![vec![0; n]; m];
    dp[0] = nxt.clone();
    for i in 1..m {
        for u in 0..n {
            dp[i][u] = dp[i - 1][dp[i - 1][u]];
        }
    }

    let mut ind: Vec<usize> = (0..n).collect();
    for i in 0..m {
        if (k >> i) & 1 == 1 {
            for u in 0..n {
                ind[u] = dp[i][ind[u]];
            }
        }
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        ans[i] = arr[ind[i]];
    }

    println!("{}", join(&ans, " "));
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
