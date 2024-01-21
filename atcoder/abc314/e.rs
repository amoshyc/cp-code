#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (n, m) = (inp[0], inp[1]);
    let mut c = vec![0; n];
    let mut p = vec![0; n];
    let mut s = vec![vec![]; n];
    for i in 0..n {
        let inp = readv::<usize>();
        c[i] = inp[0];
        p[i] = inp[1];
        s[i] = inp[2..].to_vec();
    }

    let mut dp = vec![1e18; m + 1];
    dp[m] = 0.0;
    for x in (0..m).rev() {
        for i in 0..n {
            let mut cnt = 0;
            let mut sum = 0.0;
            for j in 0..p[i] {
                if s[i][j] == 0 {
                    cnt += 1;
                } else {
                    sum += dp.get(x + s[i][j]).unwrap_or(&0.0);
                }
            }
            let rhs = ((p[i] * c[i]) as f64 + sum) / (p[i] - cnt) as f64;
            dp[x] = dp[x].min(rhs);
        }
    }

    println!("{:.6}", dp[0]);
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

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
