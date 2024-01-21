#![allow(unused)]

fn main() {
    // f(i, r) = winner after i operation and T % 7 = r
    let n = read::<usize>();
    let s = mapv(&reads(), |&c| c.to_digit(10).unwrap() as usize);
    let x = mapv(&reads(), |&x| if x == 'T' { 0 } else { 1 });

    let mut dp = vec![vec![0; 7]; n + 1];
    dp[n][0] = 0;
    for r in 1..7 {
        dp[n][r] = 1;
    }

    for i in (0..n).rev() {
        for r in 0..7 {
            let r1 = (10 * r + 0) % 7;
            let r2 = (10 * r + s[i]) % 7;
            if dp[i + 1][r1] == x[i] || dp[i + 1][r2] == x[i] {
                dp[i][r] = x[i];
            } else {
                dp[i][r] = 1 - x[i];
            }
        }
    }

    if dp[0][0] == 0 {
        println!("Takahashi")
    } else {
        println!("Aoki");
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

fn mapv<T, S>(arr: &Vec<T>, f: fn(&T) -> S) -> Vec<S> {
    arr.iter().map(f).collect()
}

fn join<T: ToString>(v: &[T], sep: &str) -> String {
    v.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(sep)
}
