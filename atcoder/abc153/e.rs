#![allow(unused)]

fn main() {
    let inp = readv::<usize>();
    let (h, n) = (inp[0], inp[1]);

    let mut v = vec![];
    let mut c = vec![];
    for _ in 0..n {
        let inp = readv::<usize>();
        v.push(inp[0]);
        c.push(inp[1]);
    }

    // dp[i, j] = minimum total cost to achieve total value j using items 0..=i
    // dp[i, j] = min(dp[i, j - v[i]] + c[i], dp[i - 1, j])

    let w = h + v.iter().max().unwrap();
    let inf = 10usize.pow(8);
    let mut dp = vec![inf; w + 1];
    dp[0] = 0;
    for i in 0..n {
        for j in v[i]..=w {
            dp[j] = dp[j].min(dp[j - v[i]] + c[i]);
        }
    }

    let ans = dp[h..].iter().min().unwrap();
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
