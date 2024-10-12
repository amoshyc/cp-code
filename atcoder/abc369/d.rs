#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<i64>();

    // dp[i, 0/1] = maximum EXP after monster 0..=i
    //              while the parity of the number of defeated monster is 0/1
    let mut dp = vec![vec![0, 0]; n];
    dp[0][0] = 0;
    dp[0][1] = arr[0];
    for i in 1..n {
        for j in 0..2 {
            let v1 = dp[i - 1][j]; // let go monster i
            let mut v2 = dp[i - 1][1 - j] + arr[i]; // defeat monster i
            if j == 0 {
                v2 += arr[i];
            }
            dp[i][j] = v1.max(v2);
        }
    }
    println!("{}", dp[n - 1].iter().max().unwrap());
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
