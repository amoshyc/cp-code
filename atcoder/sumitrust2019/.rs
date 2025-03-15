#![allow(unused)]

fn main() {
    let x = read::<usize>();

    let inf = 10i64.pow(18);
    let c = (100..=105).collect::<Vec<i64>>();
    let v = (100..=105).collect::<Vec<usize>>();
    let dp = unbounded_knapsack_by_value(v, c, x, inf);

    if dp[x] == inf {
        println!("0");
    } else {
        println!("1");
    }
}

// dp[i, j] = minimum total cost of total value j using items 0..=i
// if we use item i one time, dp[i, j] = dp[i, j - v[i]] + c[i]
// if we don't use item i, dp[i, j] = dp[i - 1, j]
fn unbounded_knapsack_by_value(v: Vec<usize>, c: Vec<i64>, p: usize, inf: i64) -> Vec<i64> {
    assert!(v.len() == c.len());
    let mut dp = vec![inf; p + 1];
    dp[0] = 0;
    for i in 0..v.len() {
        for j in v[i]..=p {
            dp[j] = dp[j].min(dp[j - v[i]] + c[i]);
        }
    }
    dp
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
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
