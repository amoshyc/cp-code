#![allow(unused)]

fn main() {
    let n = read::<usize>();
    let arr = readv::<usize>();

    let mut ans = 0;
    for b in 0..30 {
        let bits = mapv(&arr, |&x| (x >> b) & 1);
        let sum = bits.iter().sum::<usize>();
        
        // Given binary string A, find the number of substring A[i..=j] (i <= j) that has xor 1
        // dp[i, 0/1] = number of substring ending at i that has overall xor 0/1
        // answer = sum(dp[i][1] for i in 0..n) - sum(A[i])
        // dp[0][arr[0]] = 1
        // dp[i, j] = dp[i - 1][j ^ A[i]] + (1 if A[i] == j else 0)
        let mut dp = vec![0, 0];
        let mut total = 0;
        for i in 0..n {
            (dp[0], dp[1]) = (dp[0 ^ bits[i]], dp[1 ^ bits[i]]);
            dp[bits[i]] += 1;
            total += dp[1];
        }

        // (i <= j) to (i < j)
        total -= sum;

        ans += total * (1 << b);
    }
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
